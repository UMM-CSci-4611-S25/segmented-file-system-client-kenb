use segmented_file_system_client::packet::{Data, Header, Packet};
use segmented_file_system_client::packet_group::PacketGroup;

#[cfg(test)]
mod tests {

    use std::ffi::OsString;
    use std::fs;

    use super::*;
    // use crate::packet::{Header, Data};
    // use crate::errors::PacketGroupError;

    #[test]
    fn test_process_header_sets_file_name() {
        let mut packet_group = PacketGroup::default();
        let header_packet = Packet::Header(Header {
            file_id: 1,
            file_name: OsString::from("test_file"),
        });
        packet_group.process_packet(header_packet);
        assert_eq!(packet_group.file_name, Some(OsString::from("test_file")));
    }

    #[test]
    fn test_process_data_increments() {
        let mut packet_group = PacketGroup::default();
        let data_packet = Packet::Data(Data {
            file_id: 1,
            packet_number: 0,
            is_last_packet: false,
            data: vec![1, 2, 3],
        });
        packet_group.process_packet(data_packet);
        assert_eq!(packet_group.packets.len(), 1);
    }

    #[test]
    fn test_received_all_packets() {
        let mut packet_group = PacketGroup::default();
        packet_group.expected_packet_count = Some(2);
        packet_group.packets.insert(0, vec![1, 2, 3]);
        packet_group.packets.insert(1, vec![4, 5, 6]);
        assert!(packet_group.all_packets_received());
    }

    #[test]
    fn test_write_file() {
        let mut packet_group = PacketGroup::default();
        packet_group.file_name = Some(OsString::from("test_file.txt"));
        packet_group.expected_packet_count = Some(2);
        packet_group.packets.insert(0, vec![1, 2, 3]);
        packet_group.packets.insert(1, vec![4, 5, 6]);

        // Assuming write_file is implemented correctly
        assert!(packet_group.write_file().is_ok());

        //delete test file created
        fs::remove_file("test_file.txt").unwrap();
    }

    #[test]
    fn test_write_file_missing_file_name() {
        let packet_group = PacketGroup::default();
        assert!(packet_group.write_file().is_err());
    }

    #[test]
    fn test_write_file_missing_packets() {
        let mut packet_group = PacketGroup::default();
        packet_group.file_name = Some(OsString::from("test_file.txt"));
        assert!(packet_group.write_file().is_err());
    }

    #[test]
    fn test_packet_group_process_packet() {
        let mut packet_group = PacketGroup::default();

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

        packet_group.process_packet(header_packet);
        packet_group.process_packet(data_packet);

        assert_eq!(packet_group.file_name, Some(OsString::from("test_file")));
        assert_eq!(packet_group.packets.len(), 1);
        assert_eq!(packet_group.packets.get(&0), Some(&vec![1, 2, 3]));
        assert_eq!(packet_group.expected_packet_count, Some(1));
    }
}
