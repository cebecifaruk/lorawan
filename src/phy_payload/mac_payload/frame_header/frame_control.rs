pub struct DownlinkFrameControl {
    pub adr: bool,
    pub ack: bool,
    pub pending: bool,
    pub options_length: u8,
}

impl DownlinkFrameControl {
    pub fn to_bytes(&self) -> u8 {
        let mut byte = 0u8;

        if self.adr {
            byte |= 0b1000_0000;
        }

        if self.ack {
            byte |= 0b0010_0000;
        }

        if self.pending {
            byte |= 0b0001_0000;
        }

        byte | (self.options_length & 0b0000_1111)
    }

    pub fn from_bytes(byte: u8) -> Self {
        DownlinkFrameControl {
            adr: (byte & 0b1000_0000) != 0,
            ack: (byte & 0b0010_0000) != 0,
            pending: (byte & 0b0001_0000) != 0,
            options_length: byte & 0b0000_1111,
        }
    }
}

pub struct UplinkFrameControl {
    pub adr: bool,
    pub adr_ack_req: bool,
    pub ack: bool,
    pub class_b: bool,
    pub options_length: u8,
}

impl UplinkFrameControl {
    pub fn to_bytes(&self) -> [u8; 1] {
        let mut byte = 0u8;

        if self.adr {
            byte |= 0b1000_0000;
        }

        if self.adr_ack_req {
            byte |= 0b0100_0000;
        }

        if self.ack {
            byte |= 0b0010_0000;
        }

        if self.class_b {
            byte |= 0b0001_0000;
        }

        [byte | (self.options_length & 0b0000_1111)]
    }

    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        let byte = bytes[0];
        UplinkFrameControl {
            adr: (byte & 0b1000_0000) != 0,
            adr_ack_req: (byte & 0b0100_0000) != 0,
            ack: (byte & 0b0010_0000) != 0,
            class_b: (byte & 0b0001_0000) != 0,
            options_length: byte & 0b0000_1111,
        }
    }
}
