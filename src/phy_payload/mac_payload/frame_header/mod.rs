mod frame_control;
<<<<<<< HEAD
pub mod test;
=======
>>>>>>> origin/main

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameHeader {
    pub dev_addr: [u8; 4],
    pub f_ctrl: u8,
    pub f_count: u16,
    pub f_opts: [u8; 15],
}

impl FrameHeader {
    pub fn to_bytes(&self) -> [u8; 7] {
        let mut bytes = [0u8; 7];
        bytes[0..4].copy_from_slice(&self.dev_addr);
        bytes[4] = self.f_ctrl;
        // Little Endian encoding for f_count
        bytes[5] = (self.f_count & 0x00FF) as u8;
        bytes[6] = ((self.f_count >> 8) & 0x00FF) as u8;
<<<<<<< HEAD
=======
        // TODO: Lets implement f_opts part
>>>>>>> origin/main
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 7 {
            return None;
        }

        let dev_addr: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let f_ctrl = bytes[4];
        let f_count: u16 = bytes[5] as u16 | (bytes[6] as u16) << 8;
        let f_opts_length: usize = (f_ctrl & 0b0000_1111) as usize;

        let mut f_opts: [u8; 15] = [0x00; 15];

        for i in 0..f_opts_length {
<<<<<<< HEAD
            f_opts[i] = bytes[7 + i]; 
            // TODO data wiill be decypted here!
=======
            f_opts[i] = bytes[7 + i];
>>>>>>> origin/main
        }

        let result = FrameHeader {
            dev_addr,
            f_ctrl,
            f_count,
            f_opts,
        };

        Some(result)
    }

<<<<<<< HEAD

=======
>>>>>>> origin/main
    pub fn len(&self) -> usize {
        7 + (self.f_ctrl & 0b0000_1111) as usize
    }
}
<<<<<<< HEAD

=======
>>>>>>> origin/main
