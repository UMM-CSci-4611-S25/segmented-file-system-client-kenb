#[allow(unused)]
use std::{
    collections::HashMap,
    ffi::OsString,
    io::{self, Write},
};

#[allow(unused)]
use crate::packet::{
    Data,
    Header,
    Packet
};

// PacketGroup contains a file_name, expected packet count, and a map of packets
#[allow(unused)]
pub struct PacketGroup {
    pub file_name: Option<OsString>,
    pub expected_packet_count: Option<usize>,
    pub packets: HashMap<u16, Vec<u8>>,
}

#[allow(dead_code)]
impl PacketGroup {
    pub fn process_packet(){
        // Placeholder implementation
    }
    
    fn process_header(){
        // Placeholder implementation
    }

    fn process_data(){
        // Placeholder implementation
    }

    pub fn received_all_packets(&self) -> bool {
        // Placeholder implementation
        true
    }

    pub fn write_file(&self) -> io::Result<()> {
        // Placeholder implementation
        Ok(())
    }
}