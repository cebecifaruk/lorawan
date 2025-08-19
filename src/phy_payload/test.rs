use super::*;
use mac_header::*;
use mac_payload::frame_header::FrameHeader;

#[test]
fn test_simple_packet() {
    let x: PHYPayload = PHYPayload {
        header: MACHeader {
            message_type: MessageType::UnconfirmedDataUp,
            major_version: MajorVersion::LoRaWANR1,
        },
        payload: MACPayload {
            header: FrameHeader {
                dev_addr: [0x59, 0xCC, 0x65, 0x01],
                f_ctrl: 0x00,
                f_count: 0x0040,
            },
            port: Some(0x55),
            data: vec![0x03, 0x67, 0x27, 0x01],
        },
    };

    let appskey: [u8; 16] = [
        0x87, 0x07, 0x73, 0xA0, 0xB3, 0x3A, 0x62, 0x06, 0xA7, 0x29, 0xE0, 0x23, 0xF2, 0x50, 0x67,
        0xFF,
    ];

    let nwsk: [u8; 16] = [
        0xE7, 0x0D, 0x18, 0xE8, 0x38, 0x12, 0x4B, 0xDB, 0x20, 0x24, 0x8F, 0x26, 0xC0, 0x40, 0xC9,
        0x11,
    ];

    let packet = x.to_bytes(true, 0x40, &appskey, &nwsk);

    assert!(packet.len() > 0);
    assert_eq!(
        packet,
        vec![
            0x40, // PHYPayload header
            0x59, 0xCC, 0x65, 0x01, // DevAddr
            0x00, // FCtrl
            0x40, 0x00, // FCnt
            0x55, // FPort
            0x3d, 0xde, 0x1b, 0xf1, // FRMPayload
            0xc4, 0xcf, 0xd7, 0x13 // MIC
        ]
    );
}
