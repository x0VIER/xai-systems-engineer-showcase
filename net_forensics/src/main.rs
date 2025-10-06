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
                EtherTypes::Ipv6 => {
                    *protocol_counts.entry("IPv6".to_string()).or_insert(0) += 1;
                }
                EtherTypes::Arp => {
                    *protocol_counts.entry("ARP".to_string()).or_insert(0) += 1;
                }
                _ => {
                    *protocol_counts.entry("Other L3".to_string()).or_insert(0) += 1;
                }
            }
        }
    }

    println!("Packet capture complete. Generating visualization...");

    // Generate visualization
    let root = BitMapBackend::new("protocol_distribution.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let protocols: Vec<String> = protocol_counts.keys().cloned().collect();
    let max_count = *protocol_counts.values().max().unwrap_or(&0);
    let protocols_segment = protocols.into_segmented();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(20)
        .caption("Network Protocol Distribution", ("sans-serif", 40.0))
        .build_cartesian_2d(protocols_segment, 0..max_count + (max_count/10))?;

    chart.configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Packet Count")
        .x_desc("Protocol")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.filled())
            .data(protocol_counts.iter().map(|(p, c)| (p, *c))),
    )?;

    root.present()?;
    println!("Generated protocol distribution chart: protocol_distribution.png");

    Ok(())
}

