use std::collections::HashMap;

use crate::{packet::Packet, packet_group::PacketGroup};

// FileManager manages the files being received
#[derive(Default)]
pub struct FileManager {
    pub files: HashMap<u8, PacketGroup>,
}

impl FileManager {
    // checks if all packets are received for all files
    // pub fn received_all_packets(&self) -> bool {
    //     !self.files.is_empty() && self.files.values().all(|file| file.all_packets_received())
    // }

    pub fn received_all_packets(&self) -> bool {
        !self.files.is_empty()
            && self.files.values().all(|file| file.all_packets_received())
            && self.files.values().all(|file| file.file_name.is_some())
    }

    // routes packets to the correct PacketGroup
    pub fn process_packet(&mut self, packet: Packet) {
        // println!("Processing packet: {:?}", packet);

        // set file_id based on the packet type
        let file_id = match &packet {
            Packet::Header(header) => header.file_id,
            Packet::Data(data) => data.file_id,
        };

        // Find the file group for the packet and process it
        let file_group = self
            .files
            .entry(file_id)
            .or_insert_with(PacketGroup::default);
        file_group.process_packet(packet); // This is the PacketGroup process_packet method
    }

    // writes all the files that are ready to be written
    pub fn write_all_files(&self) -> Result<(), std::io::Error> {
        for file_group in self.files.values() {
            // if let Some(file_name) = &file_group.file_name {
            //     println!("Writing file: {:?}", file_name);
            // }
            file_group.write_file()?;
        }
        Ok(())
    }
}

// methods used for testing, worried about their security implications
#[allow(unused)]
impl FileManager {
    pub fn insert_packet_group(&mut self, file_id: u8, packet_group: PacketGroup) {
        self.files.insert(file_id, packet_group);
    }
    pub fn get_packet_group(&self, file_id: u8) -> Option<&PacketGroup> {
        self.files.get(&file_id)
    }
}
