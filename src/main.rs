mod phy_payload;

use phy_payload::PHYPayload;
use phy_payload::mac_header::{MACHeader, MajorVersion, MessageType};

use phy_payload::mac_payload::MACPayload;
use phy_payload::mac_payload::frame_header::FrameHeader;
use std::num::ParseIntError;

fn print_packet() {
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
                f_opts: [0x00; 15],
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

    print!("");
    for byte in bytes.iter() {
        print!("{:02x}", byte);
    }
    println!("");
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn main() {
    print_packet();

    let lines = std::io::stdin().lines();

    for line in lines {
        let line = line.unwrap();
        let buffer = decode_hex(&line);
        let buffer = buffer.unwrap();

        let payload = PHYPayload::from_bytes(&buffer);

        println!("{:#?}", payload);
    }
}
