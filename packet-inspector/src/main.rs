// 1.2 — Borrowing Rules & Aliasing (Shared XOR Mutable), NLL
// Exercise: Zero-Copy Packet Header Inspector
// Spec: see §4 of "1.2 Borrowing rules & aliasing (shared XOR mutable), NLL.md" in the notes vault.

use std::net::Ipv4Addr;

struct PacketBuffer {
    bytes: Vec<u8>,
}
struct ParsedHeader<'a> {
    src_ip: &'a [u8],
    dst_ip: &'a [u8],
    flags: u8,
}

impl PacketBuffer {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
    fn parse(&self) -> Option<ParsedHeader> {
        if self.bytes.len() != 24 {
            // Ipv4 packet size
            return None;
        }
        let header = ParsedHeader {
            src_ip: &self.bytes[12..16], // SRC IP in IPv4 packet
            dst_ip: &self.bytes[16..20], // DST IP in IPv4 packet
            flags: 8,                    // Random value
        };

        Some(header)
    }

    fn redact(&mut self, offset: usize, len: usize, mask: u8) {
        for bytes in self.bytes[offset..offset + len].iter_mut() {
            *bytes ^= mask
        }
    }
}

fn main() {
    // Sample IPv4 Packet: 20 bytes header + 4 bytes payload = 24 bytes total
    let packet_bytes: Vec<u8> = vec![
        // --- ROW 1: Version, IHL, DSCP/ECN, Total Length ---
        0x45, // Version: 4, IHL: 5 (5 * 4 = 20 bytes header)
        0x00, // DSCP / ECN: Default/Routine
        0x00, 0x18, // Total Length: 24 bytes (0x0018)
        // --- ROW 2: Identification, Flags, Fragment Offset ---
        0x1c, 0x1d, // Identification: 0x1c1d
        0x40, 0x00, // Flags: Don't Fragment (0x4000), Fragment Offset: 0
        // --- ROW 3: TTL, Protocol, Header Checksum ---
        0x40, // TTL: 64
        0x11, // Protocol: 17 (UDP)
        0x00, 0x00, // Header Checksum: (0x0000 for simplicity, usually calculated)
        // --- ROW 4: Source IP Address ---
        0xc0, 0xa8, 0x01, 0x0a, // Source IP: 192.168.1.10
        // --- ROW 5: Destination IP Address ---
        0x08, 0x08, 0x08, 0x08, // Destination IP: 8.8.8.8
        // --- PAYLOAD ---
        0xde, 0xad, 0xbe, 0xef, // 4 bytes of data
    ];

    let mut packet_buffer = PacketBuffer::new(packet_bytes);

    if let Some((header1, header2)) = packet_buffer.parse().zip(packet_buffer.parse()) {
        println!("{} {}", header1.flags, header2.flags); // both alive here, multiple readers,
        // Shared XOR Mutable rule holds
        let src_ip = Ipv4Addr::new(
            header1.src_ip[0],
            header1.src_ip[1],
            header1.src_ip[2],
            header1.src_ip[3],
        );
        // packet_buffer.redact(16, 8, 5); // Error: E0502. cant have a mutation while the reader
        // (header1) lives.
        // Shared XOR Mutable rule does not hold.
        let dst_ip = Ipv4Addr::new(
            header1.dst_ip[0],
            header1.dst_ip[1],
            header1.dst_ip[2],
            header1.dst_ip[3],
        );
        println!("SRC: {src_ip} \nDST: {dst_ip}");
        // header1 dropped here due to NLL, now we can have a mutable reference to packet_buffer
        // using redact()

        packet_buffer.redact(12, 8, 5); // redact sensitive data (ip here)
    }
}
