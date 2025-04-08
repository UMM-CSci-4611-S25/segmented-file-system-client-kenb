// Custom error types for parsing

#[derive(Debug)]
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

#[derive(Debug)]
pub enum PacketGroupError {
    MissingPacket(u16),
    IoError(std::io::Error),
    MissingFileName,
    MissingPacketCount,
}

impl std::fmt::Display for PacketGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketGroupError::MissingPacket(packet_number) => {
                write!(f, "Missing packet: {}", packet_number)
            }
            PacketGroupError::MissingFileName => write!(f, "Missing file name"),
            PacketGroupError::IoError(err) => write!(f, "IO error: {}", err),
            PacketGroupError::MissingPacketCount => write!(f, "Missing packet count"),
        }
    }
}

impl std::error::Error for PacketGroupError {}

impl From<std::io::Error> for PacketGroupError {
    fn from(err: std::io::Error) -> Self {
        PacketGroupError::IoError(err)
    }
}
impl From<PacketGroupError> for std::io::Error {
    fn from(err: PacketGroupError) -> Self {
        match err {
            PacketGroupError::IoError(io_err) => io_err,
            PacketGroupError::MissingPacket(_) => {
                std::io::Error::new(std::io::ErrorKind::Other, "Missing packet error")
            }
            PacketGroupError::MissingFileName => {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing file name")
            }
            PacketGroupError::MissingPacketCount => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing packet count")
            }
        }
    }
}
