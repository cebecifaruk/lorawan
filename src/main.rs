mod enc;
mod phy_payload;

use phy_payload::PHYPayload;
use phy_payload::mac_header::{MACHeader, MajorVersion, MessageType};

use phy_payload::mac_payload::MACPayload;
use phy_payload::mac_payload::frame_header::FrameHeader;

fn main() {
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
            data: vec![0x3d, 0xde, 0x1b, 0xf1],
        },
        mic: [0xc4, 0xcf, 0xd7, 0x13],
    };

    println!("{:02x?}", x.to_bytes());

    let appskey = [
        0x87, 0x07, 0x73, 0xA0, 0xB3, 0x3A, 0x62, 0x06, 0xA7, 0x29, 0xE0, 0x23, 0xF2, 0x50, 0x67,
        0xFF,
    ];

    println!(
        "Block 1: {:02x?}",
        enc::encrypt_payload(
            true,
            [0x59, 0xCC, 0x65, 0x01],
            0x0040,
            &[0x03, 0x67, 0x27, 0x01],
            appskey
        )
    );
}
