
use std::{convert::TryFrom, ffi::OsString};

use crate::errors::PacketParseError;

#[derive(Debug)]
#[allow(unused)]
pub enum Packet {
    Header(Header),
    Data(Data),
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub file_id: u8,
    pub file_name: OsString,
}

#[derive(Debug, PartialEq)]
pub struct Data {
    pub file_id: u8,
    pub packet_number: u16,
    pub is_last_packet: bool,
    pub data: Vec<u8>,
}

impl TryFrom<&[u8]> for Packet {
    type Error = PacketParseError;

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        // This is a placeholder implementation
        Err(PacketParseError::InvalidPacketFormat)
    }
}