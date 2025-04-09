use std::{convert::TryFrom, ffi::OsString};

use crate::errors::PacketParseError;

const MIN_PACKET_SIZE: usize = 3; // Minimum size for a valid packet

#[derive(Debug)]
pub enum Packet {
    Header(Header),
    Data(Data),
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub file_id: u8,
    pub file_name: OsString,
    pub expected_packet_count: usize,
}

#[derive(Debug, PartialEq)]
pub struct Data {
    pub file_id: u8,
    pub packet_number: u16,
    pub is_last_packet: bool,
    pub data: Vec<u8>,
}

// TryFrom implementation for Packet (Top level packet type)
impl TryFrom<&[u8]> for Packet {
    type Error = PacketParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        println!("Raw packet data: {:?}", value);

        // Check if the packet is too short
        if value.len() < MIN_PACKET_SIZE {
            return Err(PacketParseError::TooShort);
        }

        let status_byte = value[0];
        let _file_id = value[1];

        // Validate the status byte
        if status_byte & 0xFC != 0 {
            return Err(PacketParseError::InvalidPacketFormat);
        }

        // Split to use the Header try_from or Data try_from
        if (status_byte & 0x01) == 0 {
            // Header packet
            Header::try_from(value).map(Packet::Header)
        } else {
            // Data packet
            Data::try_from(value).map(Packet::Data)
        }
    }
}

// TryFrom implementation for Header
impl TryFrom<&[u8]> for Header {
    type Error = PacketParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(PacketParseError::TooShort);
        }

        let file_id = value[1];
        let file_name = OsString::from(
            String::from_utf8(value[2..].to_vec())
                .map_err(|_| PacketParseError::InvalidPacketFormat)?,
        );
        let expected_packet_count = u16::from_be_bytes([value[0], value[1]]) as usize;

        println!(
            "Parsed header packet: file_id = {}, file_name = {:?}",
            file_id, file_name
        );

        Ok(Header {
            file_id,
            file_name,
            expected_packet_count,
        })
    }
}

// TryFrom implementation for Data packet
impl TryFrom<&[u8]> for Data {
    type Error = PacketParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 4 {
            return Err(PacketParseError::TooShort);
        }

        let file_id = value[1];
        let packet_number = u16::from_be_bytes([value[2], value[3]]);
        let is_last_packet = value[0] & 0x02 != 0;
        let data = value[4..].to_vec();

        println!(
            "Parsed data packet: file_id = {}, packet_number = {}, is_last_packet = {}, data = {:?}",
            file_id, packet_number, is_last_packet, data
        );

        Ok(Data {
            file_id,
            packet_number,
            is_last_packet,
            data,
        })
    }
}
