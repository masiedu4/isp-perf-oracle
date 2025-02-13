use anyhow::{Result, Context};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{process::Command, time::Duration};
use tokio::time;
use tracing::{info, error};

#[derive(Debug, Serialize, Deserialize)]
struct NetworkMetrics {
    timestamp: i64,
    download_speed: f64,
    upload_speed: f64,
    ping_ms: f64,
    isp: String,
    server_name: String,
}

#[derive(Debug, Deserialize)]
struct SpeedTestResult {
    download: SpeedInfo,
    upload: SpeedInfo,
    ping: PingInfo,
    isp: String,
    server: ServerInfo,
}

#[derive(Debug, Deserialize)]
struct SpeedInfo {
    bandwidth: u64,
}

#[derive(Debug, Deserialize)]
struct PingInfo {
    latency: f64,
}

#[derive(Debug, Deserialize)]
struct ServerInfo {
    name: String,
}

struct NetworkMonitor;

impl NetworkMonitor {
    fn new() -> Self {
        Self
    }

    async fn collect_metrics(&self) -> Result<NetworkMetrics> {
        // Run speedtest with JSON output
        let output = Command::new("speedtest")
            .args(["--format=json", "--progress=no" , "--accept-license"])
            .output()
            .context("Failed to execute speedtest command")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Speedtest failed: {}", error);
        }

        let result: SpeedTestResult = serde_json::from_slice(&output.stdout)
            .context("Failed to parse speedtest output")?;

        // Convert bandwidth from bytes/s to Mbps
        let download_mbps = (result.download.bandwidth as f64 * 8.0) / 1_000_000.0;
        let upload_mbps = (result.upload.bandwidth as f64 * 8.0) / 1_000_000.0;

        Ok(NetworkMetrics {
            timestamp: Utc::now().timestamp(),
            download_speed: download_mbps,
            upload_speed: upload_mbps,
            ping_ms: result.ping.latency,
            isp: result.isp,
            server_name: result.server.name,
        })
    }

    async fn start_monitoring(&self) {
        let interval = Duration::from_secs(60); // 1 minute interval
        let mut interval_timer = time::interval(interval);

        loop {
            interval_timer.tick().await;
            
            info!("Starting speed test...");
            match self.collect_metrics().await {
                Ok(metrics) => {
                    info!("Network metrics collected successfully");
                    println!("\n=== Network Metrics ===");
                    println!("Timestamp: {}", metrics.timestamp);
                    println!("Download Speed: {:.2} Mbps", metrics.download_speed);
                    println!("Upload Speed: {:.2} Mbps", metrics.upload_speed);
                    println!("Ping: {:.2} ms", metrics.ping_ms);
                    println!("ISP: {}", metrics.isp);
                    println!("Server: {}", metrics.server_name);
                    println!("=====================\n");

                    // Here you would add code to send data to smart contract
                    // For example, converting the metrics to hex and sending them
                }
                Err(e) => {
                    error!("Failed to collect metrics: {}", e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let monitor = NetworkMonitor::new();
    info!("Starting network monitoring...");
    monitor.start_monitoring().await;

    Ok(())
}