use segmented_file_system_client::packet::{Header, Packet};
use segmented_file_system_client::packet_group::PacketGroup;

#[cfg(test)]
mod tests {

    use std::ffi::OsString;

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
}
