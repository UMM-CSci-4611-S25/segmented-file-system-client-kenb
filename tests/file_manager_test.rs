use segmented_file_system_client::file_manager::FileManager;
use segmented_file_system_client::packet::{Data, Header, Packet};
use segmented_file_system_client::packet_group::PacketGroup;

use std::ffi::OsString;
use std::fs;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_received_all_packets() {
        let file_manager = FileManager::default();
        assert!(file_manager.received_all_packets());
    }

    #[test]
    fn test_received_all_packets_complete() {
        let mut file_manager = FileManager::default();

        // Simulate receiving packets
        let header_packet = Packet::Header(Header {
            file_id: 1,
            file_name: OsString::from("test_file"),
        });
        let data_packet1 = Packet::Data(Data {
            file_id: 1,
            packet_number: 0,
            is_last_packet: false,
            data: vec![1, 2, 3],
        });
        let data_packet2 = Packet::Data(Data {
            file_id: 1,
            packet_number: 1,
            is_last_packet: true,
            data: vec![4, 5, 6],
        });

        file_manager.process_packet(header_packet);
        file_manager.process_packet(data_packet1);
        file_manager.process_packet(data_packet2);

        assert!(file_manager.received_all_packets());
    }

    #[test]
    fn test_received_all_packets_incomplete() {
        let mut file_manager = FileManager::default();
        // Simulate receiving a header packet
        let header_packet = Packet::Header(Header {
            file_id: 1,
            file_name: OsString::from("test_file"),
        });
        file_manager.process_packet(header_packet);

        // Simulate receiving only one data packet
        let data_packet = Packet::Data(Data {
            file_id: 1,
            packet_number: 0,
            is_last_packet: false,
            data: vec![1, 2, 3],
        });

        file_manager.process_packet(data_packet);

        // Verify that not all packets have been received
        assert!(!file_manager.received_all_packets());

        // Optionally, verify the state of the PacketGroup
        let file_group = file_manager.get_packet_group(1).unwrap();
        assert_eq!(file_group.file_name, Some(OsString::from("test_file")));
        assert_eq!(file_group.packets.len(), 1);
        assert_eq!(file_group.packets.get(&0), Some(&vec![1, 2, 3]));
    }

    #[test]
    fn test_process_packet() {
        let mut file_manager = FileManager::default();
        let header_packet = Packet::Header(Header {
            file_id: 1,
            file_name: OsString::from("test_file"),
        });
        let data_packet = Packet::Data(Data {
            file_id: 1,
            packet_number: 0,
            is_last_packet: true,
            data: vec![1, 2, 3],
        });

        file_manager.process_packet(header_packet);
        file_manager.process_packet(data_packet);

        let file_group = file_manager.get_packet_group(1).unwrap();
        assert_eq!(file_group.file_name, Some(OsString::from("test_file")));
        assert_eq!(file_group.packets.len(), 1);
        assert_eq!(file_group.packets.get(&0), Some(&vec![1, 2, 3]));
    }

    #[test]
    fn test_write_all_files() {
        let mut file_manager = FileManager::default();
        let header_packet = Packet::Header(Header {
            file_id: 1,
            file_name: OsString::from("test_file"),
        });
        let data_packet = Packet::Data(Data {
            file_id: 1,
            packet_number: 0,
            is_last_packet: true,
            data: vec![1, 2, 3],
        });
        let data_packet2 = Packet::Data(Data {
            file_id: 1,
            packet_number: 1,
            is_last_packet: false,
            data: vec![4, 5, 6],
        });

        file_manager.process_packet(header_packet);
        file_manager.process_packet(data_packet);
        file_manager.process_packet(data_packet2);

        assert!(file_manager.write_all_files().is_ok());

        // Check if the file was created and contains the expected data
        let file_contents = fs::read("test_file").unwrap();
        assert_eq!(file_contents, vec![1, 2, 3, 4, 5, 6]);

        // Clean up the test file
        fs::remove_file("test_file").unwrap();
    }

    #[test]
    fn test_get_packet_group() {
        let mut file_manager = FileManager::default();

        // Simulate adding a PacketGroup
        let mut packet_group = PacketGroup::default();
        packet_group.expected_packet_count = Some(2);
        packet_group.packets.insert(0, vec![1, 2, 3]);
        file_manager.insert_packet_group(1, packet_group);

        // Use the get_packet_group method
        let file_group = file_manager.get_packet_group(1);
        assert!(file_group.is_some());
        assert_eq!(file_group.unwrap().packets.len(), 1);
    }

    #[test]
    fn test_file_manager_process_packet() {
        let mut file_manager = FileManager::default();

        let header_packet = Packet::Header(Header {
            file_id: 1,
            file_name: OsString::from("test_file"),
        });
        let data_packet = Packet::Data(Data {
            file_id: 1,
            packet_number: 0,
            is_last_packet: true,
            data: vec![1, 2, 3],
        });

        file_manager.process_packet(header_packet);
        file_manager.process_packet(data_packet);

        let file_group = file_manager.get_packet_group(1).unwrap();
        assert_eq!(file_group.file_name, Some(OsString::from("test_file")));
        assert_eq!(file_group.packets.len(), 1);
        assert_eq!(file_group.packets.get(&0), Some(&vec![1, 2, 3]));
    }
}
