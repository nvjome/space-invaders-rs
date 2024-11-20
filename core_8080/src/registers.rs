pub struct Registers {
    pub a_reg: u8, // accumulator
    pub bc_reg: RegisterPair,
    pub de_reg: RegisterPair,
    pub hl_reg: RegisterPair,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a_reg: 0,
            bc_reg: RegisterPair {high: 0, low: 0},
            de_reg: RegisterPair {high: 0, low: 0},
            hl_reg: RegisterPair {high: 0, low: 0},
        }
    }
}

pub struct RegisterPair {
    pub high: u8,
    pub low: u8,
}

impl RegisterPair {
    pub fn get_pair(&self) -> u16 {
        ((self.high as u16) << 8) & (self.low as u16)
    }

    pub fn set_pair(&mut self, value: u16) {
        self.high = ((value & 0xFF00) >> 8) as u8;
        self.low = (value & 0x00FF) as u8;
    }
}