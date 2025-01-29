use std::net::{SocketAddr, UdpSocket};

use gran_turismo_query::*;

fn main() {
    // Create a UDP socket bound to any available address
    let socket = UdpSocket::bind("0.0.0.0:33740").expect("Failed to bind socket");

    // IP address and port to send/receive packets
    let ip_address = "192.168.1.84"; // This will be the address of your PS5
    let port = 33739; // Do NOT change this port, as the telemetry server listens to incoming packets on this port
    let destination: SocketAddr = format!("{}:{}", ip_address, port)
        .parse()
        .expect("Invalid IP address or port");

    // Send heartbeat packet to the telemetry server
    socket
        .send_to(constants::PACKET_HEARTBEAT_DATA, destination)
        .expect("Failed to send packet");

    // Send a packet
    loop {
        socket
            .send_to(constants::PACKET_HEARTBEAT_DATA, destination)
            .expect("Failed to send packet");
        let mut buf = [0u8; constants::PACKET_SIZE];
        socket
            .recv_from(&mut buf)
            .expect("Failed to receive packet");
        let packet = packet::Packet::try_from(&buf).expect("Failed to parse packet");
        println!("{:#?}", packet);
        // Send hearbeat packet every 100 packets, otherwise the telemetry server will assume this client is dead
        if packet.packet_id % 100 == 0 {
            socket
                .send_to(constants::PACKET_HEARTBEAT_DATA, destination)
                .expect("Failed to send packet");
        }
    }
}
