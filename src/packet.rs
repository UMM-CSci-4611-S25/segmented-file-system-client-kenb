use std::{convert::TryFrom, ffi::OsString};

use crate::errors::PacketParseError;

#[derive(Debug)]
pub enum Packet {
    Header(Header),
    Data(Data),
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub file_id: u8,
    pub file_name: OsString,
}

// The structure of the data packet
#[derive(Debug, PartialEq)]
pub struct Data {
    pub file_id: u8,
    pub packet_number: u16,
    pub is_last_packet: bool,
    pub data: Vec<u8>,
}

// // Check if the packet is a data packet
// fn is_data_packet(value: &[u8]) -> bool {
//     (status_byte & 0x01) != 0 // First bit indicates a data packet
// }

// // Check if the packet is the last packet
// fn is_last_packet(value: &[u8]) -> bool {
//     (status_byte & 0x02) != 0 // Second bit indicates if it's the last packet
// }

// parsing a packet
impl TryFrom<&[u8]> for Packet {
    type Error = PacketParseError;

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        println!("Raw packet data: {:?}", _value);

        // // Check if the packet is too short
        // if _value.len() < 3 {
        //     return Err(PacketParseError::TooShort);
        // }

        // Set the status_byte and file_id
        let status_byte = _value[0];
        let file_id = _value[1];

        // // Validate the status byte
        // if status_byte & 0xFC != 0 {
        //     return Err(PacketParseError::InvalidPacketFormat);
        // }

        // Check if the packet is a header or data packet
        if (status_byte & 0x01) == 0 {
            // Header packet
            let header = Header {
                file_id,
                file_name: OsString::from(
                    String::from_utf8(_value[2..].to_vec())
                        .map_err(|_| PacketParseError::InvalidPacketFormat)?,
                ),
            };
            println!("Parsed header packet: {:?}", header);
            return Ok(Packet::Header(header));
        } else {
            // Data packet
            let data = Data {
                file_id,
                packet_number: u16::from_be_bytes([_value[2], _value[3]]),
                is_last_packet: status_byte & 0x02 != 0,
                data: _value[4..].to_vec(),
            };
            println!("Parsed data packet: {:?}", data);
            return Ok(Packet::Data(data));
        }
    }
}
