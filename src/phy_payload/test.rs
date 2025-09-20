#[cfg(test)]
mod tests {

    use crate::{phy_payload, PHYPayload};
    use crate::phy_payload::mac_header::*; 
    use crate::phy_payload::mac_payload::frame_header::FrameHeader; 
    use crate::phy_payload::mac_payload::MACPayload;

    #[warn(dead_code)]
    fn dummy_frame_header() -> FrameHeader {
        FrameHeader { dev_addr: [0x26, 0x61, 0x06, 0xAA], f_ctrl: 0x00, f_count: 1, f_opts: [0; 15] }
    }

    #[warn(dead_code)]
    fn dummy_mac_payload() -> MACPayload {
        MACPayload {
            header: dummy_frame_header(),
            port: Some(1),
            data: vec![0x48, 0x65, 0x6C, 0x6C, 0x6F], // "Hello"
        }
    }

    #[warn(dead_code)]
    fn dummy_phy_payload() -> PHYPayload {
        PHYPayload {
            header: MACHeader {
                message_type: MessageType::UnconfirmedDataUp,
                major_version: MajorVersion::LoRaWANR1,
            },
            payload: dummy_mac_payload(),
        }
    }


    #[test]
    fn test_phypayload_from_bytes() {
        // MHDR (1) + FHDR (7) + FPort (1) + FRMPayload (3) = 12 byte
        let raw = vec! {
            0x40,                   // MHDR
            0x26, 0x61, 0x06, 0xAA, // DevAddr
            0x00,                   // FCtrl
            0x01, 0x00,             // FCnt
            0x01,                   // FPort
            0xAA, 0xBB, 0xCC        // FRMPayload
        };

        let phy = PHYPayload::from_bytes(&raw).unwrap(); 

        assert_eq!(phy.header.message_type, MessageType::UnconfirmedDataUp);
        assert_eq!(phy.payload.header.dev_addr, [0x26, 0x61, 0x06, 0xAA]);
        assert_eq!(phy.payload.port, Some(1));
        assert_eq!(phy.payload.data, vec![0xAA, 0xBB, 0xCC]);

    }


    #[test]
    fn test_phypayload_to_bytes_and_mic() {
        let app_skey = [0x00u8; 16]; 
        let nwk_skey = [0x01u8; 16];
        
        let phy = dummy_phy_payload();

        let bytes = phy.to_bytes(true, 1, &app_skey, &nwk_skey);

        // MHDR 
        assert_eq!(bytes[0] & 0xE0, 0x40);  // UnconfirmedDataUp (010 << 5)
        // DevAddr
        assert_eq!(&bytes[1..5], &[0x26, 0x61, 0x06, 0xAA]);
        // FPort
        assert_eq!(bytes[8], 0x01);
        // FRMPayload length ? 
        assert!(bytes.len() >= 13); // + MIC (4 byte)

        // MIC 
        let mic = &bytes[bytes.len()-4..];
        let calc_mic = phy.calculate_mic(true, 1, &app_skey, &nwk_skey);
        assert_eq!(mic, &calc_mic);
    
    }


    #[test]
    fn test_mic_consistency() {
        let app_skey = [0xA5u8; 16];
        let nwk_skey = [0xA6u8; 16];

        let phy1 = dummy_phy_payload();
        let phy2 = dummy_phy_payload();

        let mic1 = phy1.calculate_mic(true, 1, &app_skey, &nwk_skey);
        let mic2 = phy2.calculate_mic(true, 1, &app_skey, &nwk_skey);

        assert_eq!(mic1, mic2);

    }


    #[test]
    fn test_roundtrip_to_from_bytes() {
        let app_skey = [0x61u8; 16];
        let nwk_skey = [0x06u8; 16];

        let phy = dummy_phy_payload(); 
        let bytes = phy.to_bytes(true, 1, &app_skey, &nwk_skey);
        let parsed = PHYPayload::from_bytes(&bytes).unwrap();

        assert_eq!(parsed.header.message_type, phy.header.message_type);
        assert_eq!(parsed.payload.header.dev_addr, phy.payload.header.dev_addr);
        assert_eq!(parsed.payload.port, phy.payload.port);

    }

}