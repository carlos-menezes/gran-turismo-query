use crate::{
    constants::{PACKET_DECRYPTION_KEY, PACKET_SIZE},
    errors::CypherError,
};
use salsa20::cipher::{KeyIvInit, StreamCipher};
use salsa20::Salsa20;

pub fn decrypt(data: &[u8; PACKET_SIZE]) -> Result<[u8; PACKET_SIZE], CypherError> {
    // We unwrap here because it will always return `Ok` variant
    let original_init_vector = data[0x40..0x44].try_into().unwrap();

    let iv1 = u32::from_le_bytes(original_init_vector);
    let iv2 = iv1 ^ 0xDEADBEAF;
    let mut iv: [u8; 8] = [0u8; 8];
    iv[0..4].copy_from_slice(&iv2.to_le_bytes());
    iv[4..].copy_from_slice(&iv1.to_le_bytes());

    // Decrypt the packet using Salsa20 cipher
    let mut cipher = Salsa20::new(PACKET_DECRYPTION_KEY[0..32].into(), &iv.into());
    let mut decrypted_buf = *data;
    cipher.apply_keystream(&mut decrypted_buf[..]);

    Ok(decrypted_buf)
}
