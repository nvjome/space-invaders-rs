use crate::core_error::CoreError;

const MEM_SIZE: usize = 0x10000;

pub struct Memory {
    pub ram: [u8; MEM_SIZE],
    pub program_counter: u16,
    pub stack_pointer: u16,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0; MEM_SIZE],
            program_counter: 0,
            stack_pointer: 0,
        }
    }

    pub fn fetch_byte(&mut self) -> Result<u8, CoreError> {
        if (self.program_counter as usize) < self.ram.len() {
            let byte = self.ram[self.program_counter as usize];
            self.program_counter += 1;
            Ok(byte)
        } else {
            Err(CoreError::IndexError { index: self.program_counter })
        }
    }

    pub fn fetch_two_bytes(&mut self) -> Result<u16, CoreError> {
        let data_low = self.fetch_byte()?;
        let data_high = self.fetch_byte()?;
        Ok((data_high as u16) << 8 | data_low as u16)
    }

    pub fn read_byte(&self, address: u16) -> Result<u8, CoreError> {
        if (address as usize) < self.ram.len() {
            Ok(self.ram[address as usize])
        } else {
            Err(CoreError::IndexError { index: address })
        }
    }

    pub fn read_two_bytes(&self, address: u16) -> Result<u16, CoreError> {
        let data_low = self.read_byte(address)?;
        let data_high = self.read_byte(address + 1)?;
        Ok((data_high as u16) << 8 | data_low as u16)
    }

    pub fn write_byte(&mut self, address: u16, data: u8) -> Result<(), CoreError> {
        if (address as usize) < self.ram.len() {
            self.ram[address as usize] = data;
            Ok(())
        } else {
            Err(CoreError::IndexError { index: address })
        }
    }

    pub fn write_two_bytes(&mut self, address: u16, data: u16) -> Result<(), CoreError> {
        self.write_byte(address, (data & 0x00FF) as u8)?;
        self.write_byte(address + 1, ((data & 0xFF00) >> 8) as u8)?;
        Ok(())
    }

    pub fn pop_stack(&mut self) -> Result<u16, CoreError> {
        let data = self.read_two_bytes(self.stack_pointer)?;
        match self.stack_pointer.checked_add(2) {
            Some(x) => self.stack_pointer = x,
            None => return Err(CoreError::StackPointerOverflow)
        }
        Ok(data)
    }

    pub fn push_stack(&mut self, data: u16) -> Result<(), CoreError>{
        match self.stack_pointer.checked_sub(2) {
            Some(x) => self.stack_pointer = x,
            None => return Err(CoreError::StackPointerOverflow)
        }
        self.write_two_bytes(self.stack_pointer, data)?;
        Ok(())
    }
}