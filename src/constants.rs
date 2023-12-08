pub const PACKET_SIZE: usize = 0x128;
pub const PACKET_MAGIC_VALUE: u32 = 0x47375330;
pub const PACKET_DECRYPTION_KEY: &[u8] = b"Simulator Interface Packet GT7 ver 0.0";
pub const PACKET_HEARTBEAT_DATA: &[u8; 1] = b"A";
