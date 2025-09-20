// TODO: Write tests for Downlink and Uplink frame control structures
<<<<<<< HEAD
#[cfg(test)]
mod tests {
    use crate::phy_payload::mac_payload::frame_header::frame_control::UplinkFrameControl;
    use crate::phy_payload::mac_payload::frame_header::frame_control::DownlinkFrameControl;
   

    #[test]
    fn test_downlink_frame_control_roundtrip() {
        let ctrl = DownlinkFrameControl {
            adr: true, 
            ack: false, 
            pending: true, 
            options_length: 5,
        };

        let byte = ctrl.to_bytes();
        let parsed = DownlinkFrameControl::from_bytes(byte);

        assert_eq!(ctrl.adr, parsed.adr);
        assert_eq!(ctrl.ack, parsed.ack);
        assert_eq!(ctrl.pending, parsed.pending);
        assert_eq!(ctrl.options_length, parsed.options_length);

    }


    #[test]
    fn test_uplink_frame_control_roundtrip() {
        let ctrl = UplinkFrameControl {
            adr: true, 
            adr_ack_req: true, 
            ack: false, 
            class_b: true, 
            options_length: 7,
        };

        let bytes = ctrl.to_bytes();
        let parsed = UplinkFrameControl::from_bytes(bytes);

        assert_eq!(ctrl.adr ,parsed.adr);
        assert_eq!(ctrl.adr_ack_req ,parsed.adr_ack_req); 
        assert_eq!(ctrl.ack ,parsed.ack);
        assert_eq!(ctrl.class_b ,parsed.class_b);
        assert_eq!(ctrl.options_length ,parsed.options_length);

    }


    #[test]
    fn test_downlink_know_value() {
        // ADR=1, ACK=1, Pending=0, FOptsLen=3
        let byte = 0b1010_0011; 
        let parsed = DownlinkFrameControl::from_bytes(byte);

        assert!(parsed.adr);
        assert!(parsed.ack);
        assert!(!parsed.pending);
        assert_eq!(parsed.options_length ,3);
        assert_eq!(parsed.to_bytes() ,byte);

    }

    #[test]
    fn test_uplink_known_value() {
        // ADR=1, ADRACKReq=0, ACK=1, ClassB=1, FOptsLen=2
        let byte = 0b1011_0010; 
        let parsed = UplinkFrameControl::from_bytes([byte]);

        assert!(parsed.adr);
        assert!(!parsed.adr_ack_req);
        assert!(parsed.ack);
        assert!(parsed.class_b);
        assert_eq!(parsed.options_length, 2);

        assert_eq!(parsed.to_bytes()[0], byte);

    }
}
=======
>>>>>>> origin/main
