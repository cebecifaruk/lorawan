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
fn test_mac_header_to_bytes_all_message_types() {
    let cases = vec! [
        (MessageType::JoinRequest, 0x00), 
        (MessageType::JoinAccept, 0x20), 
        (MessageType::UnconfirmedDataUp, 0x40),
        (MessageType::UnconfirmedDataDown, 0x60),
        (MessageType::ConfirmedDataUp, 0x80),
        (MessageType::ConfirmedDataDown, 0xA0),
        (MessageType::RFU, 0xC0),
        (MessageType::Proprietary, 0xE0),
    ];

    for(msg_type, expected) in cases {
        let header = MACHeader {
            message_type: msg_type, 
            major_version: MajorVersion::LoRaWANR1,
        };
        let result = header.to_bytes(); 
        assert_eq!(result[0], expected);

    }

}


#[test]
fn test_mac_header_to_bytes_major_versions() {
    let cases = vec![
        (MajorVersion::LoRaWANR1, 0x00), 
        (MajorVersion::RFU1, 0x01), 
        (MajorVersion::RFU2, 0x02), 
        (MajorVersion::RFU3, 0x03),      
    ];

    for (major, expected) in cases {
        let header = MACHeader{
            message_type: MessageType::JoinRequest, 
            major_version: major, 
        };

        let result = header.to_bytes(); 
        assert_eq!(result[0], expected);
    }

}


#[test]
fn test_mac_header_from_bytes_join_request() {
    let bytes = [0x00];
    let header = MACHeader::from_bytes(&bytes);

    assert!(header.message_type == MessageType::JoinRequest);
    assert!(header.major_version == MajorVersion::LoRaWANR1);
}

// TODO: Write more tests for from_bytes

#[test]
fn test_mac_header_from_bytes_all_message_types() {
    let cases = vec![
        (0x00, MessageType::JoinRequest),
        (0x20, MessageType::JoinAccept),
        (0x40, MessageType::UnconfirmedDataUp),
        (0x60, MessageType::UnconfirmedDataDown),
        (0x80, MessageType::ConfirmedDataUp),
        (0xA0, MessageType::ConfirmedDataDown),
        (0xC0, MessageType::RFU),
        (0xE0, MessageType::Proprietary),
    ];

    for (byte, expected) in cases {
        let header = MACHeader::from_bytes(&[byte]);
        assert_eq!(header.message_type, expected);
    }
}

#[test]
fn test_mac_header_from_bytes_major_versions() {
    let cases = vec![
        (0x00, MajorVersion::LoRaWANR1),
        (0x01, MajorVersion::RFU1),
        (0x02, MajorVersion::RFU2),
        (0x03, MajorVersion::RFU3),
    ];

    for (byte, expected) in cases {
        let header = MACHeader::from_bytes(&[byte]);
        assert_eq!(header.major_version, expected);
    }
}


#[test]
fn test_mac_header_roundtrip() {
    let header = MACHeader {
        message_type: MessageType::ConfirmedDataDown, 
        major_version: MajorVersion::LoRaWANR1, 
    };

    let bytes = header.to_bytes(); 
    let parsed = MACHeader::from_bytes(&bytes);

    assert_eq!(header.message_type, parsed.message_type);
    assert_eq!(header.major_version, parsed.major_version);
}

