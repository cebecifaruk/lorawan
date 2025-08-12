pub mod frame_header;

use frame_header::FrameHeader;

#[derive(Debug)]
pub struct MACPayload {
    pub header: FrameHeader,
    pub port: Option<u8>,
    pub data: Vec<u8>,
}

impl MACPayload {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes().to_vec();

        if let Some(port) = self.port {
            bytes.push(port);
        }

        bytes.extend(self.data.iter());

        bytes
    }
}
