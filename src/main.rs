mod phy_payload;

use phy_payload::PHYPayload;
use phy_payload::mac_header::{MACHeader, MajorVersion, MessageType};

use phy_payload::mac_payload::MACPayload;
use phy_payload::mac_payload::frame_header::FrameHeader;

fn main() {
    let appskey: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];

    let nwsk: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];

    let x: PHYPayload = PHYPayload {
        header: MACHeader {
            message_type: MessageType::UnconfirmedDataUp,
            major_version: MajorVersion::LoRaWANR1,
        },
        payload: MACPayload {
            header: FrameHeader {
                dev_addr: [0x03, 0x02, 0x01, 0x00],
                f_ctrl: 0x00,
                f_count: 3,
            },
            port: Some(0x55),
            data: "Hello, World!".as_bytes().to_vec(),
        },
    };

    let bytes = x.to_bytes(true, 3, &appskey, &nwsk);

    print!("{{");
    for byte in bytes.iter() {
        print!("0x{:02x}, ", byte);
    }
    println!("}}");
}
