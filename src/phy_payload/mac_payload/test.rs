#[cfg(test)]
mod tests {
    use crate::MACPayload;
    use crate::phy_payload::mac_payload::frame_header::FrameHeader;

    #[warn(dead_code)]
    fn dummy_frame_header() -> FrameHeader {
        FrameHeader { dev_addr: [0x26, 0x61, 0x06, 0xAA], f_ctrl: 0x00, f_count: 1, f_opts: [0; 15] }
    }

    #[test]
    fn test_macpayload_from_bytes() {
        // FHDR = 7 byte (DevAddr + FCtrl + FCnt)
        // DevAddr = AA 06 61 26
        // FCtrl = 00
        // FCnt = 01 00
        // FPort = 01
        // FRMPayload = [0x48, 0x65, 0x6C] = "Hel"
        let raw = vec![
            0x26, 0x61, 0x06, 0xAA, // DevAddr
            0x00,                   // FCtrl
            0x01, 0x00,             // FCnt
            0x01,                   // FPort
            0x48, 0x65, 0x6C,       // FRMPayload
        ];

        let payload = MACPayload::from_bytes(&raw).unwrap();
        assert_eq!(payload.header.dev_addr, [0x26, 0x61, 0x06, 0xAA]);
        assert_eq!(payload.header.f_count, 1);
        assert_eq!(payload.port, Some(0x01));
        assert_eq!(payload.data, vec![0x48, 0x65, 0x6C]);  
    }


    #[test]
    fn test_macpayload_to_bytes_without_encryption() {

        // Dummy key = all zero
        let key = [0x00u8; 16];

        let payload = MACPayload {
            header: dummy_frame_header(), 
            port: Some(1), 
            data: vec![0x01, 0x02, 0x03],
        };

        let bytes = payload.to_bytes(true, 1, &key);

        // FHDR (7 byte) + FPort (1) + FRMPayload (3)
        assert_eq!(bytes.len(), 11);

        // DevAddr
        assert_eq!(&bytes[0..4], &[0x26, 0x61, 0x06, 0xAA]);

        // FCtrl 
        assert_eq!(bytes[4], 0x00);

        // FCnt 
        assert_eq!(&bytes[5..7], &[0x01, 0x00]);

        // FPort
        assert_eq!(bytes[7], 0x01);

    }


    #[test]
    fn test_calculate_block_i() {
        let dev_addr = [0x01, 0x02, 0x03, 0x04];

        let f_count = 0xAABBCCDD; 

        let block = MACPayload::calculate_block_i(true, dev_addr, f_count, 1);

        assert_eq!(block[0], 0x01); // 0x01
        assert_eq!(block[5], 0x00); // uplink flag
        assert_eq!(&block[6..10], &[0x01, 0x02, 0x03, 0x4]);    // DevAddr
        assert_eq!(&block[10..14], &[0xDD, 0xCC, 0xBB, 0xAA]);  // f_count
        assert_eq!(block[15], 0x01);    // block index

    }


    #[test]
    fn test_encrypted_payload_length() {
        let key = [0x00u8; 16];

        let payload = MACPayload {
            header: dummy_frame_header(),
            port: Some(1),
            data: (0..30).collect(), // 30 byte payload
        };

        let encrypted = payload.get_encrypted_payload(true, payload.header.dev_addr, 1, &key);

        assert_eq!(encrypted.len(), 30);
    }

    #[test]
    fn test_roundtrip_to_from_bytes() {
        let key = [0x00u8; 16];

        let payload = MACPayload {
            header: dummy_frame_header(),
            port: Some(10),
            data: vec![0xAB, 0xCD, 0xEF],
        };

        let bytes = payload.to_bytes(true, 1, &key);
        let parsed = MACPayload::from_bytes(&bytes).unwrap();

        assert_eq!(parsed.header.dev_addr, payload.header.dev_addr);
        assert_eq!(parsed.port, Some(10));
        assert_eq!(parsed.data.len(), payload.data.len());
    }


}