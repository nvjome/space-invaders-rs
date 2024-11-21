pub struct ShiftRegister {
    register: u16,
    offset: u8,
}

impl ShiftRegister {
    pub fn new() -> Self {
        Self {
            register: 0x0000,
            offset: 0x00,
        }
    }

    pub fn load(&mut self, data: u8) {
        self.register >>= 8;
        self.register |= (data as u16) << 8;
    }
    
    pub fn set_offset(&mut self, offset: u8) {
        self.offset = offset & 0x07;
    }

    pub fn get_shift(&self) -> u8 {
        (((self.register << self.offset) & 0xFF00) >> 8) as u8
    }
}