pub mod mac_header;
pub mod mac_payload;

use aes::Aes128;
use cmac::{Cmac, Mac};
use mac_header::MACHeader;
use mac_payload::MACPayload;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct PHYPayload {
    pub header: MACHeader,
    pub payload: MACPayload,
}

impl PHYPayload {
    pub fn to_bytes(
        &self,
        is_up_link: bool,
        f_count: u32,
        appsk: &[u8; 16],
        nwsk: &[u8; 16],
    ) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(self.header.to_bytes()[0]);
        bytes.extend(self.payload.to_bytes(is_up_link, f_count, appsk));
        let mic = self.calculate_mic(true, f_count, appsk, nwsk);
        bytes.extend(mic.iter());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let header = MACHeader::from_bytes(bytes);

        let payload = MACPayload::from_bytes(&bytes[1..]);

        if payload.is_none() {
            return None;
        }

        let payload = payload.unwrap();

        // TODO: We should check mic code also!

        return Some(PHYPayload { header, payload });
    }

    pub fn get_mic_msg(&self, is_up_link: bool, f_count: u32, appsk: &[u8; 16]) -> Vec<u8> {
        let mut msg: Vec<u8> = Vec::new();
        msg.push(self.header.to_bytes()[0]);
        msg.extend(self.payload.to_bytes(is_up_link, f_count, appsk));
        msg
    }

    fn get_mic_b0(is_up_link: bool, dev_addr: [u8; 4], f_cnt: u32, msg_len: u8) -> [u8; 16] {
        [
            0x49,
            0x00,
            0x00,
            0x00,
            0x00,
            if is_up_link { 0x00 } else { 0x01 },
            dev_addr[0],
            dev_addr[1],
            dev_addr[2],
            dev_addr[3],
            (f_cnt & 0xFF) as u8,
            ((f_cnt >> 8) & 0xFF) as u8,
            ((f_cnt >> 16) & 0xFF) as u8,
            ((f_cnt >> 24) & 0xFF) as u8,
            0x00,
            msg_len,
        ]
    }

    pub fn calculate_mic(
        &self,
        is_up_link: bool,
        f_count: u32,
        appsk: &[u8; 16],
        nwsk: &[u8; 16],
    ) -> [u8; 4] {
        let mut mac = Cmac::<Aes128>::new_from_slice(nwsk).unwrap();

        let dev_addr = self.payload.header.dev_addr;
        let f_cnt = self.payload.header.f_count;

        let msg = self.get_mic_msg(is_up_link, f_count, appsk);
        let b0 = Self::get_mic_b0(is_up_link, dev_addr, f_cnt as u32, msg.len() as u8);

        mac.update(&b0);
        mac.update(&msg);

        let result = mac.finalize().into_bytes();
        let cmac = result.as_slice();

        return [cmac[0], cmac[1], cmac[2], cmac[3]];
    }
}
