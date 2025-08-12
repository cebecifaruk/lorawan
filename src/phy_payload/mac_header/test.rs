use super::*;

#[test]
fn test_mac_header_to_bytes_join_request() {
    let header = MACHeader {
        message_type: MessageType::JoinRequest,
        major_version: MajorVersion::LoRaWANR1,
    };

    let result = header.to_bytes();

    assert!(result[0] == 0x00);
}

#[test]
fn test_mac_header_to_bytes_join_accept() {
    let header = MACHeader {
        message_type: MessageType::JoinAccept,
        major_version: MajorVersion::LoRaWANR1,
    };

    let result = header.to_bytes();

    assert!(result[0] == 0x20);
}

#[test]
fn test_mac_header_to_bytes_unconfirmed_data_up() {
    let header = MACHeader {
        message_type: MessageType::UnconfirmedDataUp,
        major_version: MajorVersion::LoRaWANR1,
    };

    let result = header.to_bytes();

    assert!(result[0] == 0x40);
}

// TODO: Write more tests for to_bytes

#[test]
fn test_mac_header_from_bytes_join_request() {
    let bytes = [0x00];
    let header = MACHeader::from_bytes(bytes);

    assert!(header.message_type == MessageType::JoinRequest);
    assert!(header.major_version == MajorVersion::LoRaWANR1);
}

// TODO: Write more tests for from_bytes
