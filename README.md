# Decentralized ISP Performance Oracle

A blockchain-based network performance monitoring system that uses Raspberry Pi to collect ISP metrics, processes them through Cartesi CoProcessor, and posts verified results on-chain.


## Overview

The Decentralized ISP Performance Oracle provides transparent, tamper-resistant ISP performance metrics by:
- Collecting network performance data every minute using a Raspberry Pi
- Processing raw metrics through Cartesi CoProcessor for verification
- Posting validated results on-chain for public access
- Maintaining continuous monitoring for data integrity

## Architecture

### Components
1. **Data Collection Layer**
   - Raspberry Pi running network monitor
   - Speedtest CLI for metrics gathering
   - Local data storage for raw measurements

2. **Processing Layer**
   - Cartesi CoProcessor for data validation
   - Statistical analysis of network metrics
   - Anomaly detection systems

3. **Blockchain Layer**
   - Smart contracts for data storage
   - On-chain verification mechanisms
   - Public access endpoints

### Data Flow
```
Raspberry Pi → Raw Data Collection → Cartesi Processing → Blockchain Storage
     ↑                                                          ↓
Continuous                                                 Public Access
Monitoring                                                   Interface
```

### Why Raspberry Pi and Ethernet?

Using a Raspberry Pi with a wired Ethernet connection provides some advantages over a laptop with Wi-Fi for network monitoring. The Raspberry Pi offers a dedicated hardware that can run 24/7 with consistent performance characteristics, while the wired connection eliminates variables like signal strength fluctuations and interference that could affect measurement accuracy. 


## Requirements

### Hardware
- Raspberry Pi 5
- Ethernet cable (CAT6 recommended)
- Stable power supply
- Micro SD card (32GB+ recommended)
- Optional: Case for Raspberry Pi

### Software
- [Raspberry Pi OS (64-bit recommended)](https://www.raspberrypi.com/documentation/computers/os.html)
- [Rust toolchain](https://rustup.rs/)
- [Speedtest CLI](https://www.speedtest.net/apps/cli)
- [Cartesi CoProcessor](https://docs.mugen.builders/cartesi-co-processor-tutorial/introduction)

### Network
- Static IP configuration recommended
- Direct connection to ISP router
- Minimum 1Mbps upload speed for reliable data transmission

## Installation

### 1. Repository Setup
```bash
# Clone the repository
git clone https://github.com/masiedu4/isp-perf-oracle
cd isp-perf-oracle

# Navigate to the network monitor directory
cd network-monitor
```

### 2. Speedtest CLI Installation(For Raspberry Pi)
```bash
# Update package lists
sudo apt-get update

# Install curl
sudo apt-get install curl

# Add Ookla's repository
curl -s https://packagecloud.io/install/repositories/ookla/speedtest-cli/script.deb.sh | sudo bash

# Install Speedtest
sudo apt-get install speedtest
```

### 3. Building the Project

#### Option A: Cross-Compilation
```bash
# Install cross-compilation tool
cargo install cross

# Build for Raspberry Pi
cross build --release --target aarch64-unknown-linux-gnu

# Copy from host machine to Raspberry Pi (replace IP with your host name or Pi's address, and <directory> with your desired path)
scp target/aarch64-unknown-linux-gnu/release/network-monitor <username>@<hostname>:/<directory>


# Example
scp target/aarch64-unknown-linux-gnu/release/network-monitor michaelasiedu@raspberrypi:/home/michaelasiedu/Code/
```

#### Option B: Direct Build on Raspberry Pi
```bash
# On the Raspberry Pi
cargo build --release
```




## Usage

### Starting the Network Monitor on Raspberry Pi
```bash
./network-monitor
```

### Example Output
```
=== Network Metrics ===
Timestamp: 1739464223
Download Speed: 229.34 Mbps
Upload Speed: 18.48 Mbps
Ping: 42.04 ms
ISP: Starlink
Server: iNETCOM Ghana
=====================
```


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.