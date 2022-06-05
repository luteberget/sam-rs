use crate::Params;


pub struct FormantTables {
    mouth: [u8; 80],
    throat: [u8; 80],
}

impl Default for FormantTables {
    fn default() -> Self {
        let mouth = [
            0x00, 0x13, 0x13, 0x13, 0x13, 0xA, 0xE, 0x12, 0x18, 0x1A, 0x16, 0x14, 0x10, 0x14, 0xE,
            0x12, 0xE, 0x12, 0x12, 0x10, 0xC, 0xE, 0xA, 0x12, 0xE, 0xA, 8, 6, 6, 6, 6, 0x11, 6, 6,
            6, 6, 0xE, 0x10, 9, 0xA, 8, 0xA, 6, 6, 6, 5, 6, 0, 0x12, 0x1A, 0x14, 0x1A, 0x12, 0xC,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xA, 0xA, 6, 6, 6, 0x2C, 0x13,
        ];
        let throat = [
            0x00, 0x43, 0x43, 0x43, 0x43, 0x54, 0x48, 0x42, 0x3E, 0x28, 0x2C, 0x1E, 0x24, 0x2C,
            0x48, 0x30, 0x24, 0x1E, 0x32, 0x24, 0x1C, 0x44, 0x18, 0x32, 0x1E, 0x18, 0x52, 0x2E,
            0x36, 0x56, 0x36, 0x43, 0x49, 0x4F, 0x1A, 0x42, 0x49, 0x25, 0x33, 0x42, 0x28, 0x2F,
            0x4F, 0x4F, 0x42, 0x4F, 0x6E, 0x00, 0x48, 0x26, 0x1E, 0x2A, 0x1E, 0x22, 0x1A, 0x1A,
            0x1A, 0x42, 0x42, 0x42, 0x6E, 0x6E, 0x6E, 0x54, 0x54, 0x54, 0x1A, 0x1A, 0x1A, 0x42,
            0x42, 0x42, 0x6D, 0x56, 0x6D, 0x54, 0x54, 0x54, 0x7F, 0x7F,
        ];

        let freqdata = [
            0x00, 0x5B, 0x5B, 0x5B, 0x5B, 0x6E, 0x5D, 0x5B, 0x58, 0x59, 0x57, 0x58, 0x52, 0x59,
            0x5D, 0x3E, 0x52, 0x58, 0x3E, 0x6E, 0x50, 0x5D, 0x5A, 0x3C, 0x6E, 0x5A, 0x6E, 0x51,
            0x79, 0x65, 0x79, 0x5B, 0x63, 0x6A, 0x51, 0x79, 0x5D, 0x52, 0x5D, 0x67, 0x4C, 0x5D,
            0x65, 0x65, 0x79, 0x65, 0x79, 0x00, 0x5A, 0x58, 0x58, 0x58, 0x58, 0x52, 0x51, 0x51,
            0x51, 0x79, 0x79, 0x79, 0x70, 0x6E, 0x6E, 0x5E, 0x5E, 0x5E, 0x51, 0x51, 0x51, 0x79,
            0x79, 0x79, 0x65, 0x65, 0x70, 0x5E, 0x5E, 0x5E, 0x08, 0x01,
        ];

        Self { mouth, throat }
    }
}

impl FormantTables {
    pub fn from_params(params: &Params) -> Self {
        Default::default() // TODO modify from parameters
    }
}