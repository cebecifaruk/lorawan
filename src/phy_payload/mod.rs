pub mod mac_header;
pub mod mac_payload;

use mac_header::MACHeader;
use mac_payload::MACPayload;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct PHYPayload {
    pub header: MACHeader,
    pub payload: MACPayload,
    pub mic: [u8; 4],
}

impl PHYPayload {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(self.header.to_bytes()[0]);
        bytes.extend(self.payload.to_bytes());
        bytes.extend(self.mic.iter());
        bytes
    }
}
