// Custom error types for parsing

#[derive(Debug)]
#[allow(unused)]
pub enum PacketParseError {
    TooShort,
    InvalidPacketFormat,
}

#[allow(unused)]
#[derive(Debug)]
pub enum ClientError {
    IoError(std::io::Error),
    PacketParseError(PacketParseError),
}

impl From<std::io::Error> for ClientError {
    fn from(e: std::io::Error) -> Self {
        ClientError::IoError(e)
    }
}

impl From<PacketParseError> for ClientError {
    fn from(e: PacketParseError) -> Self {
        Self::PacketParseError(e)
    }
}
