use std::collections::HashMap;

use crate::{packet::Packet, packet_group::PacketGroup};

// FileManager is responsible for managing the files being received
#[derive(Default)]
pub struct FileManager {
    pub files: HashMap<u8, PacketGroup>,
}

impl FileManager {
    // pub fn received_all_packets(&self) -> bool {
    //     println!("Checking if all packets are received...");

    //     if self.files.is_empty() {
    //         println!("No files to process.");
    //         return true;
    //     }

    //     for (file_id, file_group) in &self.files {
    //         println!(
    //             "File ID: {}, Received: {}",
    //             file_id,
    //             file_group.all_packets_received()
    //         );
    //     }
    //     self.files.len() == 0 || self.files.values().all(|file| file.all_packets_received())
    // }
    // pub fn received_all_packets(&self) -> bool {
    //     println!("Checking if all packets are received...");

    //     if let Some(expected) = self.expected_packets {
    //         println!(
    //             "Received packets: {}, Expected packets: {}",
    //             self.received_packets, expected
    //         );
    //         return self.received_packets >= expected;
    //     }

    //     println!("Expected packet count is not set. Returning false.");
    //     false
    // }
    pub fn received_all_packets(&self) -> bool {
        if self.files.len() < 3 {
            println!("Haven’t received all packets yet");
            return false;
        }

        self.files.values().all(|file| file.all_packets_received())
    }

    pub fn process_packet(&mut self, packet: Packet) {
        println!("Processing packet: {:?}", packet);

        let file_id = match &packet {
            Packet::Header(header) => header.file_id,
            Packet::Data(data) => data.file_id,
        };

        let file_group = self
            .files
            .entry(file_id)
            .or_insert_with(PacketGroup::default);
        file_group.process_packet(packet);
    }

    // pub fn process_packet(&mut self, _packet: Packet) {
    //     println!("Processing packet: {:?}", _packet);

    //     let file_id = match &_packet {
    //         Packet::Header(header) => {
    //             self.expected_packets = Some(header.expected_packet_count);
    //             header.file_id
    //         }
    //         Packet::Data(data) => data.file_id,
    //     };

    //     self.received_packets += 1;

    //     let file_group = self
    //         .files
    //         .entry(file_id)
    //         .or_insert_with(PacketGroup::default);
    //     file_group.process_packet(_packet);
    // }

    pub fn write_all_files(&self) -> Result<(), std::io::Error> {
        for file_group in self.files.values() {
            if let Some(file_name) = &file_group.file_name {
                println!("Writing file: {:?}", file_name);
            }
            file_group.write_file()?;
        }
        Ok(())
    }
}

// impl Default for FileManager {
//     fn default() -> Self {
//         FileManager {
//             files: HashMap::new(),
//         }
//     }
// }

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
