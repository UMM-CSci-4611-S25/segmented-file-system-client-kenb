use segmented_file_system_client::errors::PacketParseError;
use segmented_file_system_client::packet::Packet;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn test_parse_header_packet() {
        let raw_data: &[u8] = &[0, 1, b't', b'e', b's', b't'];
        let packet = Packet::try_from(raw_data).unwrap();
        if let Packet::Header(header) = packet {
            assert_eq!(header.file_id, 1);
            assert_eq!(header.file_name, OsString::from("test"));
        } else {
            panic!("Expected Header packet");
        }
    }

    #[test]
    fn test_parse_data_packet() {
        let raw_data: &[u8] = &[1, 1, 0, 1, b'd', b'a', b't', b'a'];
        let packet = Packet::try_from(raw_data).unwrap();
        if let Packet::Data(data) = packet {
            assert_eq!(data.file_id, 1);
            assert_eq!(data.packet_number, 1);
            assert!(data.is_last_packet);
            assert_eq!(data.data, b"data".to_vec());
        } else {
            panic!("Expected Data packet");
        }
    }

    #[test]
    fn test_invalid_packet_format() {
        let raw_data: &[u8] = &[0, 1]; // Too short
        let result = Packet::try_from(raw_data);
        assert!(matches!(result, Err(PacketParseError::TooShort)));
    }

    #[test]
    fn test_invalid_utf8_in_header() {
        let raw_data: &[u8] = &[0, 1, 0xFF, 0xFE, 0xFD]; // Invalid UTF-8 bytes
        let result = Packet::try_from(raw_data);
        assert!(matches!(result, Err(PacketParseError::InvalidPacketFormat)));
    }

    #[test]
    fn test_invalid_status_byte() {
        let raw_data: &[u8] = &[0xFF, 1, 0, 0, b'd', b'a', b't', b'a']; // Invalid status byte
        let result = Packet::try_from(raw_data);
        // No explicit error for invalid status byte, but it should not panic
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_data_packet() {
        let raw_data: &[u8] = &[1, 1, 0, 0]; // Valid header but no data
        let packet = Packet::try_from(raw_data).unwrap();
        if let Packet::Data(data) = packet {
            assert_eq!(data.file_id, 1);
            assert_eq!(data.packet_number, 0);
            assert_eq!(data.is_last_packet, false);
            assert!(data.data.is_empty());
        } else {
            panic!("Expected Data packet");
        }
    }

    #[test]
    fn test_last_data_packet() {
        let raw_data: &[u8] = &[3, 1, 0, 1, b'd', b'a', b't', b'a']; // Last packet (status_byte & 0x02 != 0)
        let packet = Packet::try_from(raw_data).unwrap();
        if let Packet::Data(data) = packet {
            assert_eq!(data.file_id, 1);
            assert_eq!(data.packet_number, 1);
            assert!(data.is_last_packet);
            assert_eq!(data.data, b"data".to_vec());
        } else {
            panic!("Expected Data packet");
        }
    }

    #[test]
    fn test_maximum_packet_size() {
        let mut raw_data = vec![1, 1, 0, 1]; // Data packet header
        raw_data.extend(vec![b'x'; 1024]); // 1024 bytes of data
        let packet = Packet::try_from(raw_data.as_slice()).unwrap(); // Convert Vec<u8> to &[u8]
        if let Packet::Data(data) = packet {
            assert_eq!(data.file_id, 1);
            assert_eq!(data.packet_number, 1);
            assert_eq!(data.is_last_packet, false);
            assert_eq!(data.data.len(), 1024);
        } else {
            panic!("Expected Data packet");
        }
    }
}
