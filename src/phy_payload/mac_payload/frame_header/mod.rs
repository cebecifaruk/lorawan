mod frame_control;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameHeader {
    pub dev_addr: [u8; 4],
    pub f_ctrl: u8,
    pub f_count: u16,
}

impl FrameHeader {
    pub fn to_bytes(&self) -> [u8; 7] {
        let mut bytes = [0u8; 7];
        bytes[0..4].copy_from_slice(&self.dev_addr);
        bytes[4] = self.f_ctrl;
        // Little Endian encoding for f_count
        bytes[5] = (self.f_count & 0x00FF) as u8;
        bytes[6] = ((self.f_count >> 8) & 0x00FF) as u8;
        bytes
    }
}
