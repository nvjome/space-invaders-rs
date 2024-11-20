const MEM_SIZE: usize = 0x10000;

pub struct Memory {
    pub ram: [u8; MEM_SIZE],
    pub program_counter: u16,
    pub stack_pointer: u16,
}

impl Memory {
    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.ram[self.program_counter as usize];
        self.program_counter += 1;
        byte
    }

    pub fn fetch_two_bytes(&mut self) -> u16 {
        let data_low = self.fetch_byte();
        let data_high = self.fetch_byte();
        (data_high as u16) << 8 & data_low as u16
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn read_two_bytes(&self, address: u16) -> u16 {
        let data_low = self.ram[address as usize];
        let data_high = self.ram[address as usize + 1];
        (data_high as u16) << 8 & data_low as u16
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }

    pub fn write_two_bytes(&mut self, address: u16, data: u16) {
        self.ram[address as usize] = (data & 0x00FF) as u8;
        self.ram[address as usize + 1] = ((data & 0xFF00) >> 8) as u8;
    }

    pub fn pop_stack(&mut self) -> u16 {
        let data = self.read_two_bytes(self.stack_pointer);
        self.stack_pointer += 2;
        data
    }

    pub fn push_stack(&mut self, data: u16) {
        self.stack_pointer -= 2;
        self.write_two_bytes(self.stack_pointer, data);
    }
}