
use std::collections::HashMap;

use crate::{
    packet::Packet,
    packet_group::PacketGroup,
};

// FileManager is responsible for managing the files being received
#[allow(unused)]
pub struct FileManager {
    files: HashMap<u8, PacketGroup>,
}

impl FileManager {
    pub fn received_all_packets(&self) -> bool {
        // Placeholder implementation
        todo!()
    }

    pub fn process_packet(&mut self, _packet: Packet) {
        // Placeholder implementation
        todo!()
    }

    pub fn write_all_files(&self) -> Result<(), std::io::Error> {
        // Placeholder implementation
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
