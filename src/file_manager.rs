use std::collections::HashMap;

use crate::{packet::Packet, packet_group::PacketGroup};

// FileManager is responsible for managing the files being received
pub struct FileManager {
    files: HashMap<u8, PacketGroup>,
}

impl FileManager {
    pub fn received_all_packets(&self) -> bool {
        self.files.len() == 0 || self.files.values().all(|file| file.received_all_packets())
    }

    pub fn process_packet(&mut self, _packet: Packet) {
        let file_id = match &_packet {
            Packet::Header(header) => header.file_id,
            Packet::Data(data) => data.file_id,
        };

        let file_group = self
            .files
            .entry(file_id)
            .or_insert_with(PacketGroup::default);
        file_group.process_packet(_packet);
    }

    pub fn write_all_files(&self) -> Result<(), std::io::Error> {
        for file_group in self.files.values() {
            file_group.write_file()?;
        }
        Ok(())
    }
}

impl Default for FileManager {
    fn default() -> Self {
        FileManager {
            files: HashMap::new(),
        }
    }
}
