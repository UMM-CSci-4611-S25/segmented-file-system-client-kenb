use std::{collections::HashMap, ffi::OsString, fs::File, io::Write};

use crate::errors::PacketGroupError;
use crate::packet::{Data, Header, Packet};

// PacketGroup contains a file_name, expected packet count, and a map of packets
pub struct PacketGroup {
    pub file_name: Option<OsString>,
    pub expected_packet_count: Option<usize>,
    pub packets: HashMap<u16, Vec<u8>>,
}

impl Default for PacketGroup {
    fn default() -> Self {
        Self {
            file_name: None,
            expected_packet_count: None,
            packets: HashMap::new(),
        }
    }
}

// PacketGroup is responsible for processing packets and writing files
impl PacketGroup {
    // process_packet processes a packet and updates the file name and packet count
    pub fn process_packet(&mut self, packet: Packet) {
        match packet {
            Packet::Header(header) => {
                println!("Processing header: {:?}", header.file_name);
                self.process_header(header);
            }
            Packet::Data(data) => {
                println!("Processing data packet: {:?}", data.packet_number);
                self.process_data(data);
            }
        }
    }

    // process_header processes a header packet and sets the file name
    fn process_header(&mut self, header: Header) {
        println!("Processing header: {:?}", header.file_name);
        self.file_name = Some(header.file_name);
    }

    // process_data processes a data packet and increments the expected packet count
    fn process_data(&mut self, data: Data) {
        self.packets.insert(data.packet_number, data.data);
        if data.is_last_packet {
            self.expected_packet_count = Some((data.packet_number + 1) as usize);
        }
    }

    // received_all_packets checks if all packets have been received
    pub fn received_all_packets(&self) -> bool {
        match self.expected_packet_count {
            Some(expected_count) => self.packets.len() == expected_count,
            None => false,
        }
    }

    // write_all_files writes all packets to the file
    pub fn write_file(&self) -> Result<(), PacketGroupError> {
        let file_name = self
            .file_name
            .as_ref()
            .ok_or(PacketGroupError::MissingFileName)?;

        // Check if all expected packets are present
        if let Some(expected_count) = self.expected_packet_count {
            for packet_number in 0..expected_count as u16 {
                if !self.packets.contains_key(&packet_number) {
                    return Err(PacketGroupError::MissingPacket(packet_number));
                }
            }
        } else {
            // If expected packet count is not set, we cannot check for missing packets
            return Err(PacketGroupError::MissingPacketCount);
        }

        let mut file = File::create(file_name)?;
        let mut packet_count: Vec<u16> = self.packets.keys().cloned().collect();
        packet_count.sort();

        // For each packet number, write the data to the file
        for packet_number in packet_count {
            if let Some(data) = self.packets.get(&packet_number) {
                file.write_all(data)?;
            }
        }
        Ok(())
    }
}
