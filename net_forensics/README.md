# Network Forensics CLI

<div align="center">
  <img src="https://github.com/x0VIER/xai-systems-engineer-showcase/raw/master/docs/net_forensics_architecture.png" alt="Network Forensics CLI Architecture" width="700">
</div>

<div align="center">
  <strong>A powerful command-line tool for capturing, analyzing, and visualizing network traffic patterns in constrained environments.</strong>
</div>

<br />

<div align="center">
  <a href="https://github.com/x0VIER/xai-systems-engineer-showcase/blob/master/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT">
  </a>
  <a href="https://github.com/topics/network-analysis">
    <img src="https://img.shields.io/badge/Feature-Network_Analysis-yellow" alt="Feature: Network Analysis">
  </a>
  <a href="https://github.com/topics/packet-capture">
    <img src="https://img.shields.io/badge/Feature-Packet_Capture-green" alt="Feature: Packet Capture">
  </a>
  <a href="https://github.com/topics/data-visualization">
    <img src="https://img.shields.io/badge/Feature-Data_Visualization-orange" alt="Feature: Data Visualization">
  </a>
</div>

<br />

## Overview

The Network Forensics CLI provides security professionals and network administrators with a lightweight yet powerful tool for investigating network traffic. It captures packets from specified interfaces, performs deep packet inspection, and generates insightful visualizations of protocol distributions and traffic patterns.

## Key Features

- **Packet Capture:** Efficiently captures network packets from any interface
- **Protocol Analysis:** Parses and identifies Ethernet, IPv4, IPv6, TCP, UDP, ICMP, and other protocols
- **Traffic Visualization:** Generates clear, informative charts of protocol distribution
- **Configurable Limits:** Controls the number of packets captured to manage resource usage
- **Lightweight Design:** Optimized for use in resource-constrained environments

## Implementation

```rust
use pcap::Device;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use clap::Parser;
use std::collections::HashMap;
use plotters::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interface: String,

    #[arg(short, long, default_value_t = 100)]
    packet_limit: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut cap = Device::from(args.interface.as_str()).open().unwrap();
    let mut packet_count = 0;
    let mut protocol_counts = HashMap::new();

    println!("Capturing {} packets on interface {}...", args.packet_limit, args.interface);

    while let Ok(packet) = cap.next_packet() {
        if packet_count >= args.packet_limit {
            break;
        }
        packet_count += 1;

        if let Some(ethernet_packet) = EthernetPacket::new(packet.data) {
            match ethernet_packet.get_ethertype() {
                EtherTypes::Ipv4 => {
                    if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                        let protocol = match ipv4_packet.get_next_level_protocol() {
                            pnet::packet::ip::IpNextHeaderProtocols::Tcp => "TCP",
                            pnet::packet::ip::IpNextHeaderProtocols::Udp => "UDP",
                            pnet::packet::ip::IpNextHeaderProtocols::Icmp => "ICMP",
                            _ => "Other L4",
                        };
                        *protocol_counts.entry(protocol.to_string()).or_insert(0) += 1;
                    }
                }
                // Handle other protocols...
            }
        }
    }

    // Generate visualization
    let root = BitMapBackend::new("protocol_distribution.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Visualization code...

    Ok(())
}
```

## Technical Implementation

The tool is built using several high-quality Rust crates:

- **libpcap bindings:** Via the `pcap` crate for efficient packet capture
- **Packet parsing:** Using `pnet` for deep packet inspection and protocol identification
- **Data visualization:** With the `plotters` crate for generating high-quality charts
- **CLI interface:** Built with `clap` for a user-friendly command-line experience

## Usage

### Basic Packet Capture

To capture packets and generate a protocol distribution visualization:

```bash
./target/debug/net_forensics --interface <interface_name> --packet-limit <number_of_packets>
```

### Parameters

- `--interface` or `-i`: The network interface to capture packets from (e.g., eth0, wlan0)
- `--packet-limit` or `-p`: The maximum number of packets to capture (default: 100)

### Example

```bash
./target/debug/net_forensics --interface eth0 --packet-limit 500
```

This will capture 500 packets from the eth0 interface and generate a visualization of the protocol distribution.

## Output

The tool generates a `protocol_distribution.png` file showing the distribution of different network protocols:

<div align="center">
  <img src="https://github.com/x0VIER/xai-systems-engineer-showcase/raw/master/net_forensics/protocol_distribution.png" alt="Protocol Distribution" width="600">
</div>

## Use Cases

- **Network Troubleshooting:** Quickly identify unusual protocol patterns
- **Security Monitoring:** Detect anomalous traffic that might indicate an attack
- **Performance Analysis:** Understand the composition of network traffic
- **Educational Tool:** Learn about network protocols and traffic patterns

## Future Enhancements

- Add packet filtering capabilities using BPF expressions
- Implement flow tracking for connection-oriented analysis
- Add time-series visualization of traffic patterns
- Support for exporting data in various formats (CSV, JSON)
- Add deep packet inspection for application-layer protocols
