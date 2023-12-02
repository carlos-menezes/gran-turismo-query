use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsePacketError {
    #[error("{} is not a valid magic value", .0)]
    InvalidMagicValue(u32),
    #[error("failed to read data from packet")]
    ReadError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum CypherError {
    #[error("{} is not a valid magic value", .0)]
    InvalidMagicValue(u32),
}
