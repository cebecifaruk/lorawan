#![allow(dead_code)]

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct MACHeader {
    pub message_type: MessageType,
    pub major_version: MajorVersion,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    JoinRequest = 0b000,
    JoinAccept = 0b001,
    UnconfirmedDataUp = 0b010,
    UnconfirmedDataDown = 0b011,
    ConfirmedDataUp = 0b100,
    ConfirmedDataDown = 0b101,
    RFU = 0b110,
    Proprietary = 0b111,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MajorVersion {
    LoRaWANR1 = 0b00,
    RFU1 = 0b01,
    RFU2 = 0b10,
    RFU3 = 0b11,
}

impl MACHeader {
    pub fn to_bytes(&self) -> [u8; 1] {
        let message_type: u8 = (self.message_type as u8 & 0b0000_0111) << 5;
        let major_version: u8 = (self.major_version as u8 & 0b0000_0011) << 0;

        return [message_type | major_version];
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let message_type = match (bytes[0] >> 5) & 0b0000_0111 {
            0b000 => MessageType::JoinRequest,
            0b001 => MessageType::JoinAccept,
            0b010 => MessageType::UnconfirmedDataUp,
            0b011 => MessageType::UnconfirmedDataDown,
            0b100 => MessageType::ConfirmedDataUp,
            0b101 => MessageType::ConfirmedDataDown,
            0b110 => MessageType::RFU,
            _ => MessageType::Proprietary,
        };

        let major_version = match bytes[0] & 0b0000_0011 {
            0b00 => MajorVersion::LoRaWANR1,
            0b01 => MajorVersion::RFU1,
            0b10 => MajorVersion::RFU2,
            _ => MajorVersion::RFU3,
        };

        MACHeader {
            message_type,
            major_version,
        }
    }
}
