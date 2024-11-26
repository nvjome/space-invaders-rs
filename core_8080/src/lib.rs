mod registers;
mod memory;
mod shift_register;
mod condition_flags;
mod io;
mod core_error;

use io::Inputs;
use shift_register::ShiftRegister;
use registers::Registers;
use memory::Memory;
use condition_flags::ConditionFlags;
use core_error::CoreError;

const SR_0_ADDR: u16 = 0x0000;
const SR_1_ADDR: u16 = 0x0008;
const SR_2_ADDR: u16 = 0x0010;
const SR_3_ADDR: u16 = 0x0018;
const SR_4_ADDR: u16 = 0x0020;
const SR_5_ADDR: u16 = 0x0028;
const SR_6_ADDR: u16 = 0x0030;
const SR_7_ADDR: u16 = 0x0038;

const ROM_ADDR: u16 = 0x0000;

pub struct CPU {
    memory: Memory,
    registers: Registers,
    flags: ConditionFlags,
    shifter: ShiftRegister,
    interrupt_enable: bool,
    pub input: Inputs,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
            flags: ConditionFlags::new(),
            interrupt_enable: false,
            shifter: ShiftRegister::new(),
            input: Inputs::new(),
        }
    }

    pub fn load_rom(&mut self, buffer: &Vec<u8>) {
        let start = ROM_ADDR as usize;
        let end = (ROM_ADDR as usize) + buffer.len();
        self.memory.ram[start..end].copy_from_slice(&buffer);
        self.memory.program_counter = ROM_ADDR;
    }

    pub fn tick(&mut self) -> Result<u32, CoreError>{
        let opcode = self.memory.fetch_byte()?;
        //println!("{:#04x}", opcode);
        self.execute(opcode)
    }

    pub fn interrupt(&mut self, interrupt: u8) {
        let _ = match interrupt {
            0 => self.execute(0xC7),
            1 => self.execute(0xCF),
            2 => self.execute(0xD7),
            3 => self.execute(0xDF),
            4 => self.execute(0xE7),
            5 => self.execute(0xEF),
            6 => self.execute(0xF7),
            7 => self.execute(0xFF),
            _ => Ok(0)
        };
    }

    fn generate_psw(&self) -> u16 { // Could break apart save and restore to seperate Registers and ConditionFlags methods
        let data_l = (self.flags.zero as u8) |
            (self.flags.sign as u8) << 1 |
            (self.flags.parity as u8) << 2 |
            (self.flags.carry as u8) << 3;
        (self.registers.a_reg as u16) << 8 | (data_l as u16)
    }

    fn restore_psw(&mut self, psw: u16) {
        self.registers.a_reg = ((psw & 0xFF00) >> 8) as u8;
        self.flags.zero = (psw & 0x01) == 0x01;
        self.flags.parity = (psw & 0x02) == 0x01;
        self.flags.carry = (psw & 0x03) == 0x01;
    }

    fn execute(&mut self, opcode: u8) -> Result<u32, CoreError> {
        let mut cycles = 1;
        
        // Super big and ugly match statement because I'm not sure of a better way
        match opcode {
            // ****** Data Transfer Group ******
            // *** Loads ***
            0x01 => { // LXI B,d16
                self.registers.bc_reg.set_pair(self.memory.fetch_two_bytes()?);
                cycles = 3;
            },
            0x11 => { // LXI D,d16
                self.registers.de_reg.set_pair(self.memory.fetch_two_bytes()?);
                cycles = 3;
            },
            0x21 => { // LXI H,d16
                self.registers.hl_reg.set_pair(self.memory.fetch_two_bytes()?);
                cycles = 3;
            },
            0x31 => { // LXI SP,d16
                self.memory.stack_pointer = self.memory.fetch_two_bytes()?;
                cycles = 3;
            },
            0x0A => { // LDAX B
                let addr = self.registers.bc_reg.get_pair();
                self.registers.a_reg = self.memory.read_byte(addr)?;
                cycles = 2;
            },
            0x1A => { // LDAX D
                let addr = self.registers.de_reg.get_pair();
                self.registers.a_reg = self.memory.read_byte(addr)?;
                cycles = 2;
            },
            0x2A => { // LHLD a16
                let addr = self.memory.fetch_two_bytes()?;
                self.registers.hl_reg.set_pair(self.memory.read_two_bytes(addr)?);
                cycles = 5;
            },
            0x3A => { // LDA a16
                let addr = self.memory.fetch_two_bytes()?;
                self.registers.a_reg = self.memory.read_byte(addr)?;
                cycles = 4;
            },
            0x06 => { // MVI B,d8
                self.registers.bc_reg.high = self.memory.fetch_byte()?;
                cycles = 2;
                },
            0x0E => { // MVI C,d8
                self.registers.bc_reg.low = self.memory.fetch_byte()?;
                cycles = 2;
            }
            0x16 => { // MVI D,d8
                self.registers.de_reg.high = self.memory.fetch_byte()?;
                cycles = 2;
                },
            0x1E => { // MVI E,d8
                self.registers.de_reg.low = self.memory.fetch_byte()?;
                cycles = 2;
            },
            0x26 => { // MVI H,d8
                self.registers.hl_reg.high = self.memory.fetch_byte()?;
                cycles = 2;
            },
            0x2E => { // MVI L,d8
                self.registers.hl_reg.low = self.memory.fetch_byte()?;
                cycles = 2;
            },
            0x36 => { // MVI M,d8
                let data = self.memory.fetch_byte()?;
                let addr = self.registers.hl_reg.get_pair();
                self.memory.write_byte(addr, data)?;
                cycles = 3;
            },
            0x3E => { // MVI A,d8
                self.registers.a_reg = self.memory.fetch_byte()?;
                cycles = 2;
            },

            // *** Stores ***
            0x02 => { // STAX B
                self.memory.write_byte(self.registers.bc_reg.get_pair(), self.registers.a_reg)?;
                cycles = 2;
            },
            0x12 => { // STAX D
                self.memory.write_byte(self.registers.de_reg.get_pair(), self.registers.a_reg)?;
                cycles = 2;
            },
            0x22 => { // SHLD a16
                let addr = self.memory.fetch_two_bytes()?;
                self.memory.write_two_bytes(addr, self.registers.hl_reg.get_pair())?;
                cycles = 5;
            },
            0x32 => { // STA a16
                let addr = self.memory.fetch_two_bytes()?;
                self.memory.write_byte(addr, self.registers.a_reg)?;
                cycles = 4;
            },

            // *** Moves ***
            0x40 => (), // MOV B,B
            0x41 => { // MOV B,C
                self.registers.bc_reg.high = self.registers.bc_reg.low;
            },
            0x42 => { // MOV B,D
                self.registers.bc_reg.high = self.registers.de_reg.high;
            },
            0x43 => { // MOV B,E
                self.registers.bc_reg.high = self.registers.de_reg.low;
            },
            0x44 => { // MOV B,H
                self.registers.bc_reg.high = self.registers.hl_reg.high;
            },
            0x45 => { // MOV B,L
                self.registers.bc_reg.high = self.registers.hl_reg.low;
            },
            0x46 => { // MOV B,M
                self.registers.bc_reg.high = self.memory.read_byte(self.registers.hl_reg.get_pair())?;
                cycles = 2;
            },
            0x47 => { // MOV B,A
                self.registers.bc_reg.high = self.registers.a_reg;
            },
            0x48 => { // MOV C,B
                self.registers.bc_reg.low = self.registers.bc_reg.high;
            },
            0x49 => (), // MOV C,C
            0x4A => { // MOV C,D
                self.registers.bc_reg.low = self.registers.de_reg.high;
            },
            0x4B => { // MOV C,E
                self.registers.bc_reg.low = self.registers.de_reg.low;
            },
            0x4C => { // MOV C,H
                self.registers.bc_reg.low = self.registers.hl_reg.high;
            },
            0x4D => { // MOV C,L
                self.registers.bc_reg.low = self.registers.hl_reg.low;
            },
            0x4E => { // MOV C,M
                self.registers.bc_reg.low = self.memory.read_byte(self.registers.hl_reg.get_pair())?;
                cycles = 2;
            },
            0x4F => { // MOV C,A
                self.registers.bc_reg.low = self.registers.a_reg;
            },
            0x50 => { // MOV D,B
                self.registers.de_reg.high = self.registers.bc_reg.high;
            },
            0x51 => { // MOV D,C
                self.registers.de_reg.high = self.registers.bc_reg.low;
            },
            0x52 => (), // MOV D,D
            0x53 => { // MOV D,E
                self.registers.de_reg.high = self.registers.de_reg.low;
            },
            0x54 => { // MOV D,H
                self.registers.de_reg.high = self.registers.hl_reg.high;
            },
            0x55 => { // MOV D,L
                self.registers.de_reg.high = self.registers.hl_reg.low;
            },
            0x56 => { // MOV D,M
                self.registers.de_reg.high = self.memory.read_byte(self.registers.hl_reg.get_pair())?;
                cycles = 2;
            },
            0x57 => { // MOV D,A
                self.registers.de_reg.high = self.registers.a_reg;
            },
            0x58 => { // MOV E,B
                self.registers.de_reg.low = self.registers.bc_reg.high;
            },
            0x59 => { // MOV E,C
                self.registers.de_reg.low = self.registers.bc_reg.low;
            },
            0x5A => { // MOV E,D
                self.registers.de_reg.low = self.registers.de_reg.high;
            },
            0x5B => (), // MOV E,E
            0x5C => { // MOV E,H
                self.registers.de_reg.low = self.registers.hl_reg.high;
            },
            0x5D => { // MOV E,L
                self.registers.de_reg.low = self.registers.hl_reg.low;
            },
            0x5E => { // MOV E,M
                self.registers.de_reg.low = self.memory.read_byte(self.registers.hl_reg.get_pair())?;
                cycles = 2;
            },
            0x5F => { // MOV E,A
                self.registers.de_reg.low = self.registers.a_reg;
            },
            0x60 => { // MOV H,B
                self.registers.hl_reg.high = self.registers.bc_reg.high;
            },
            0x61 => { // MOV H,C
                self.registers.hl_reg.high = self.registers.bc_reg.low;
            },
            0x62 => { // MOV H,D
                self.registers.hl_reg.high = self.registers.de_reg.high;
            },
            0x63 => { // MOV H,E
                self.registers.hl_reg.high = self.registers.de_reg.low;
            },
            0x64 => (), // MOV H,H
            0x65 => { // MOV H,L
                self.registers.hl_reg.high = self.registers.hl_reg.low;
            },
            0x66 => { // MOV H,M
                self.registers.hl_reg.high = self.memory.read_byte(self.registers.hl_reg.get_pair())?;
                cycles = 2;
            },
            0x67 => { // MOV H,A
                self.registers.hl_reg.high = self.registers.a_reg;
            },
            0x68 => { // MOV L,B
                self.registers.hl_reg.low = self.registers.bc_reg.high;
            },
            0x69 => { // MOV L,C
                self.registers.hl_reg.low = self.registers.bc_reg.low;
            },
            0x6A => { // MOV L,D
                self.registers.hl_reg.low = self.registers.de_reg.high;
            },
            0x6B => { // MOV L,E
                self.registers.hl_reg.low = self.registers.bc_reg.low;
            },
            0x6C => { // MOV L,H
                self.registers.hl_reg.low = self.registers.hl_reg.high;
            },
            0x6D => (), // MOV L,L
            0x6E => { // MOV L,M
                self.registers.hl_reg.low = self.memory.read_byte(self.registers.hl_reg.get_pair())?;
                cycles = 2;
            },
            0x6F => { // MOV L,A
                self.registers.hl_reg.low = self.registers.a_reg;
            },
            0x70 => { // MOV M,B
                let addr = self.registers.hl_reg.get_pair();
                self.memory.write_byte(addr, self.registers.bc_reg.high)?;
                cycles = 2;
            },
            0x71 => { // MOV M,C
                let addr = self.registers.hl_reg.get_pair();
                self.memory.write_byte(addr, self.registers.bc_reg.low)?;
                cycles = 2;
            },
            0x72 => { // MOV M,D
                let addr = self.registers.hl_reg.get_pair();
                self.memory.write_byte(addr, self.registers.de_reg.high)?;
                cycles = 2;
            },
            0x73 => { // MOV M,E
                let addr = self.registers.hl_reg.get_pair();
                self.memory.write_byte(addr, self.registers.de_reg.low)?;
                cycles = 2;
            },
            0x74 => { // MOV M,H
                let addr = self.registers.hl_reg.get_pair();
                self.memory.write_byte(addr, self.registers.hl_reg.high)?;
                cycles = 2;
            },
            0x75 => { // MOV M,L
                let addr = self.registers.hl_reg.get_pair();
                self.memory.write_byte(addr, self.registers.hl_reg.low)?;
                cycles = 2;
            },
            0x77 => { // MOV M,A
                let addr = self.registers.hl_reg.get_pair();
                self.memory.write_byte(addr, self.registers.a_reg)?;
                cycles = 2;
            },
            0x78 => { // MOV A,B
                self.registers.a_reg = self.registers.bc_reg.high;
            },
            0x79 => { // MOV A,C
                self.registers.a_reg = self.registers.bc_reg.low;
            },
            0x7A => { // MOV A,D
                self.registers.a_reg = self.registers.de_reg.high;
            },
            0x7B => { // MOV A,E
                self.registers.a_reg = self.registers.de_reg.low;
            },
            0x7C => { // MOV A,H
                self.registers.a_reg = self.registers.hl_reg.high;
            },
            0x7D => { // MOV A,L
                self.registers.a_reg = self.registers.hl_reg.low;
            },
            0x7E => { // MOV A,M
                let addr = self.registers.hl_reg.get_pair();
                self.registers.a_reg = self.memory.read_byte(addr)?;
                cycles = 2;
            },
            0x7F => (), // MOV A,A

            // ****** Arithmetic Group ******
            // *** Increments ***
            0x03 => { // INX B
                self.registers.bc_reg.set_pair(self.registers.bc_reg.get_pair().wrapping_add(1));
            },
            0x13 => { // INX D
                self.registers.de_reg.set_pair(self.registers.de_reg.get_pair().wrapping_add(1));
            },
            0x23 => { // INX D
                self.registers.hl_reg.set_pair(self.registers.hl_reg.get_pair().wrapping_add(1));
            },
            0x33 => { // INX SP
                self.memory.stack_pointer = self.memory.stack_pointer.wrapping_add(1);
            },
            0x04 => { // INR B
                self.registers.bc_reg.high = self.registers.bc_reg.high.wrapping_add(1);
                self.flags.zero = self.registers.bc_reg.high == 0;
                self.flags.sign = self.registers.bc_reg.high & 0x80 != 0;
                self.flags.parity = parity(self.registers.bc_reg.high);
            },
            0x0C => { // INR C
                self.registers.bc_reg.low = self.registers.bc_reg.low.wrapping_add(1);
                self.flags.zero = self.registers.bc_reg.low == 0;
                self.flags.sign = self.registers.bc_reg.low & 0x80 != 0;
                self.flags.parity = parity(self.registers.bc_reg.low);
            },
            0x14 => { // INR D
                self.registers.de_reg.high = self.registers.de_reg.high.wrapping_add(1);
                self.flags.zero = self.registers.de_reg.high == 0;
                self.flags.sign = self.registers.de_reg.high & 0x80 != 0;
                self.flags.parity = parity(self.registers.de_reg.high);
            },
            0x1C => { // INR E
                self.registers.de_reg.low = self.registers.de_reg.low.wrapping_add(1);
                self.flags.zero = self.registers.de_reg.low == 0;
                self.flags.sign = self.registers.de_reg.low & 0x80 != 0;
                self.flags.parity = parity(self.registers.de_reg.low);
            },
            0x24 => { // INR H
                self.registers.hl_reg.high = self.registers.hl_reg.high.wrapping_add(1);
                self.flags.zero = self.registers.hl_reg.high == 0;
                self.flags.sign = self.registers.hl_reg.high & 0x80 != 0;
                self.flags.parity = parity(self.registers.hl_reg.high);
            },
            0x2C => { // INR L
                self.registers.hl_reg.low = self.registers.hl_reg.low.wrapping_add(1);
                self.flags.zero = self.registers.hl_reg.low == 0;
                self.flags.sign = self.registers.hl_reg.low & 0x80 != 0;
                self.flags.parity = parity(self.registers.hl_reg.low);
            },
            0x34 => { // INR M
                let addr = self.registers.hl_reg.get_pair();
                let data_old = self.memory.read_byte(addr)?;
                let data_new = data_old.wrapping_add(1);
                self.memory.write_byte(addr, data_new)?;
                self.flags.zero = data_new == 0;
                self.flags.sign = data_new & 0x80 != 0;
                self.flags.parity = parity(data_new);
                cycles = 3;
            },
            0x3C => { // INR A
                self.registers.a_reg = self.registers.a_reg.wrapping_add(1);
                self.flags.zero = self.registers.a_reg == 0;
                self.flags.sign = self.registers.a_reg & 0x80 != 0;
                self.flags.parity = parity(self.registers.a_reg);
            },

            // *** Decrements ***
            0x05 => { // DCR B
                self.registers.bc_reg.high = self.registers.bc_reg.high.wrapping_sub(1);
                self.flags.zero = self.registers.bc_reg.high == 0;
                self.flags.sign = self.registers.bc_reg.high & 0x80 != 0;
                self.flags.parity = parity(self.registers.bc_reg.high);
            },
            0x0D => { // DCR C
                self.registers.bc_reg.low = self.registers.bc_reg.low.wrapping_sub(1);
                self.flags.zero = self.registers.bc_reg.low == 0;
                self.flags.sign = self.registers.bc_reg.low & 0x80 != 0;
                self.flags.parity = parity(self.registers.bc_reg.low);
            },
            0x15 => { // DCR D
                self.registers.de_reg.high = self.registers.de_reg.high.wrapping_sub(1);
                self.flags.zero = self.registers.de_reg.high == 0;
                self.flags.sign = self.registers.de_reg.high & 0x80 != 0;
                self.flags.parity = parity(self.registers.de_reg.high);
            },
            0x1D => { // DCR E
                self.registers.de_reg.low = self.registers.de_reg.low.wrapping_sub(1);
                self.flags.zero = self.registers.de_reg.low == 0;
                self.flags.sign = self.registers.de_reg.low & 0x80 != 0;
                self.flags.parity = parity(self.registers.de_reg.low);
            },
            0x25 => { // DCR H
                self.registers.hl_reg.high = self.registers.hl_reg.high.wrapping_sub(1);
                self.flags.zero = self.registers.hl_reg.high == 0;
                self.flags.sign = self.registers.hl_reg.high & 0x80 != 0;
                self.flags.parity = parity(self.registers.hl_reg.high);
            },
            0x2D => { // DCR L
                self.registers.hl_reg.low = self.registers.hl_reg.low.wrapping_sub(1);
                self.flags.zero = self.registers.hl_reg.low == 0;
                self.flags.sign = self.registers.hl_reg.low & 0x80 != 0;
                self.flags.parity = parity(self.registers.hl_reg.low);
            },
            0x35 => { // DCR M
                let addr = self.registers.hl_reg.get_pair();
                let data_old = self.memory.read_byte(addr)?;
                let data_new = data_old.wrapping_sub(1);
                self.memory.write_byte(addr, data_new)?;
                self.flags.zero = data_new == 0;
                self.flags.sign = data_new & 0x80 != 0;
                self.flags.parity = parity(data_new);
                cycles = 3;
            },
            0x3D => { // DCR A
                self.registers.a_reg = self.registers.a_reg.wrapping_sub(1);
                self.flags.zero = self.registers.a_reg == 0;
                self.flags.sign = self.registers.a_reg & 0x80 != 0;
                self.flags.parity = parity(self.registers.a_reg);
            },
            0x0B => { //DCX B
                self.registers.bc_reg.set_pair(self.registers.bc_reg.get_pair().wrapping_sub(1));
            },
            0x1B => { //DCX D
                self.registers.de_reg.set_pair(self.registers.de_reg.get_pair().wrapping_sub(1));
            },
            0x2B => { //DCX H
                self.registers.hl_reg.set_pair(self.registers.hl_reg.get_pair().wrapping_sub(1));
            },
            0x3B => { //DCX SP
                self.memory.stack_pointer = self.memory.stack_pointer.wrapping_sub(1);
            },

            // *** Double Adds ***
            0x09 => { // DAD B
                let (result, carry) = (self.registers.hl_reg.get_pair() as u16).overflowing_add(self.registers.bc_reg.get_pair() as u16);
                self.registers.hl_reg.set_pair(result);
                self.flags.carry = carry;
                cycles = 3;
            },
            0x19 => { // DAD D
                let (result, carry) = (self.registers.hl_reg.get_pair() as u16).overflowing_add(self.registers.de_reg.get_pair() as u16);
                self.registers.hl_reg.set_pair(result);
                self.flags.carry = carry;
                cycles = 3;
            },
            0x29 => { // DAD H
                let (result, carry) = (self.registers.hl_reg.get_pair() as u16).overflowing_add(self.registers.hl_reg.get_pair() as u16);
                self.registers.hl_reg.set_pair(result);
                self.flags.carry = carry;
                cycles = 3;
            },
            0x39 => { // DAD SP
                let (result, carry) = (self.registers.hl_reg.get_pair() as u16).overflowing_add(self.memory.stack_pointer);
                self.registers.hl_reg.set_pair(result);
                self.flags.carry = carry;
                cycles = 3;
            },

            // ****** Logic Group ******
            0x07 => { // RLC
                self.registers.a_reg = self.registers.a_reg.rotate_left(1);
                self.flags.carry = self.registers.a_reg & 0x01 == 0x01;
            },
            0x0F => { // RRC
                self.registers.a_reg = self.registers.a_reg.rotate_right(1);
                self.flags.carry = self.registers.a_reg & 0x80 == 0x80;
            },
            0x17 => { // RAL
                let temp = self.registers.a_reg;
                self.registers.a_reg = (temp << 1) | (self.flags.carry as u8);
                self.flags.carry = temp & 0x80 == 0x80;
            },
            0x1F => { // RAR
                let temp = self.registers.a_reg;
                self.registers.a_reg = ((self.flags.carry as u8) << 7 ) | (temp >> 1);
                self.flags.carry = temp & 0x01 == 0x01;
            },
            0x27 => { // DAA
                let mut data_l = self.registers.a_reg & 0x0F;
                let mut data_h = self.registers.a_reg >> 4;
                let mut carry = false;
                if data_l > 9 {
                    data_l += 6;
                    if data_l > 0x0F {
                        data_h += data_l >> 4;
                    }
                }
                if (data_h > 9)  | (self.flags.carry == true) {
                    data_h += 6;
                    if data_h > 0x0F { carry = true }
                }
                self.registers.a_reg = (data_h << 4) | (data_l & 0x0F);
                self.flags.carry = carry;
                self.flags.zero = self.registers.a_reg == 0;
                self.flags.sign = self.registers.a_reg & 0x80 != 0;
                self.flags.parity = parity(self.registers.a_reg);
            },
            0x2F => { // CMA
                self.registers.a_reg = !self.registers.a_reg;
            },
            0x37 => { // STC
                self.flags.carry = true;
            },
            0x3F => { // CMC
                self.flags.carry = !self.flags.carry;
            },
            0xE6 => { // ANI d8
                self.registers.a_reg &= self.memory.fetch_byte()?;
                self.flags.carry = false;
                self.flags.zero = self.registers.a_reg == 0;
                self.flags.sign = self.registers.a_reg & 0x80 != 0;
                self.flags.parity = parity(self.registers.a_reg);
                cycles = 2;
            },
            0xEE => { // XRI d8
                self.registers.a_reg ^= self.memory.fetch_byte()?;
                self.flags.carry = false;
                self.flags.zero = self.registers.a_reg == 0;
                self.flags.sign = self.registers.a_reg & 0x80 != 0;
                self.flags.parity = parity(self.registers.a_reg);
                cycles = 2;
            },
            0xF6 => { // ORI d8
                self.registers.a_reg &= self.memory.fetch_byte()?;
                self.flags.carry = false;
                self.flags.zero = self.registers.a_reg == 0;
                self.flags.sign = self.registers.a_reg & 0x80 != 0;
                self.flags.parity = parity(self.registers.a_reg);
                cycles = 2;
            },
            0xFE => { // CPI d8
                let (result, carry) = self.registers.a_reg.overflowing_sub(self.memory.fetch_byte()?);
                self.flags.carry = carry;
                self.flags.zero = result == 0;
                self.flags.sign = result & 0x80 != 0;
                self.flags.parity = parity(result);
                cycles = 2;
            },

            // ****** Branch Group ******
            // *** Returns ***
            0xC9 => { // RET
                self.memory.program_counter = self.memory.pop_stack()?;
                cycles = 3;
            },
            0xC0 => { // RNZ
                if self.flags.zero == false {
                    self.memory.program_counter = self.memory.pop_stack()?;
                }
            },
            0xC8 => { // RZ
                if self.flags.zero == true {
                    self.memory.program_counter = self.memory.pop_stack()?;
                }
            },
            0xD0 => { // RNC
                if self.flags.carry == false {
                    self.memory.program_counter = self.memory.pop_stack()?;
                }
            },
            0xD8 => { // RC
                if self.flags.carry == true {
                    self.memory.program_counter = self.memory.pop_stack()?;
                }
            },
            0xE0 => { // RPO
                if self.flags.parity == false {
                    self.memory.program_counter = self.memory.pop_stack()?;
                }
            },
            0xE8 => { // RPE
                if self.flags.parity == true {
                    self.memory.program_counter = self.memory.pop_stack()?;
                }
            },
            0xF0 => { // RP
                if self.flags.sign == false {
                    self.memory.program_counter = self.memory.pop_stack()?;
                }
            },
            0xF8 => { // RM
                if self.flags.sign == true {
                    self.memory.program_counter = self.memory.pop_stack()?;
                }
            },
            
            // *** Jumps ***
            0xC3 => { //JMP a16
                self.memory.program_counter = self.memory.fetch_two_bytes()?;
                cycles = 3;
            },
            0xC2 => { // JNZ a16
                if self.flags.zero == false {
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xCA => { // JZ a16
                if self.flags.zero == true {
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xD2 => { // JNC a16
                if self.flags.carry == false {
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xDA => { // JC a16
                if self.flags.carry == true {
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xE2 => { // JPO a16
                if self.flags.parity == false {
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xEA => { // JPE a16
                if self.flags.parity == true {
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xF2 => { // JP a16
                if self.flags.sign == false {
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xFA => { // JM a16
                if self.flags.sign == true {
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },

            // *** Calls ***
            0xC4 => { // CNZ a16
                if self.flags.zero == false {
                    self.memory.push_stack(self.memory.program_counter)?;
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xCC => { // CZ a16
                if self.flags.zero == true {
                    self.memory.push_stack(self.memory.program_counter)?;
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xD4 => { // CNC a16
                if self.flags.carry == false {
                    self.memory.push_stack(self.memory.program_counter)?;
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xDC => { // CC a16
                if self.flags.carry == true {
                    self.memory.push_stack(self.memory.program_counter)?;
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xE4 => { // CPO a16
                if self.flags.parity == false {
                    self.memory.push_stack(self.memory.program_counter)?;
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xEC => { // CPE a16
                if self.flags.parity == true {
                    self.memory.push_stack(self.memory.program_counter)?;
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xF4 => { // CP a16
                if self.flags.sign == false {
                    self.memory.push_stack(self.memory.program_counter)?;
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xFC => { // CM a16
                if self.flags.sign == true {
                    self.memory.push_stack(self.memory.program_counter)?;
                    self.memory.program_counter = self.memory.fetch_two_bytes()?;
                }
                cycles = 3;
            },
            0xCD => { // CALL a16
                self.memory.push_stack(self.memory.program_counter)?; // Push program counter to stack
                self.memory.program_counter = self.memory.fetch_two_bytes()?; // Set program counter to new address
                cycles = 5;
            }

            // *** Subroutines ***
            0xC7 => { // RST 0
                self.memory.push_stack(self.memory.program_counter)?; // Push program counter to stack
                self.memory.program_counter = SR_0_ADDR; // Set program counter to new address
                cycles = 3;
            },
            0xCF => { // RST 1
                self.memory.push_stack(self.memory.program_counter)?;
                self.memory.program_counter = SR_1_ADDR;
                cycles = 3;
            },
            0xD7 => { // RST 2
                self.memory.push_stack(self.memory.program_counter)?;
                self.memory.program_counter = SR_2_ADDR;
                cycles = 3;
            },
            0xDF => { // RST 3
                self.memory.push_stack(self.memory.program_counter)?;
                self.memory.program_counter = SR_3_ADDR;
                cycles = 3;
            },
            0xE7 => { // RST 4
                self.memory.push_stack(self.memory.program_counter)?;
                self.memory.program_counter = SR_4_ADDR;
                cycles = 3;
            },
            0xEF => { // RST 5
                self.memory.push_stack(self.memory.program_counter)?;
                self.memory.program_counter = SR_5_ADDR;
                cycles = 3;
            },
            0xF7 => { // RST 6
                self.memory.push_stack(self.memory.program_counter)?;
                self.memory.program_counter = SR_6_ADDR;
                cycles = 3;
            },
            0xFF => { // RST 7
                self.memory.push_stack(self.memory.program_counter)?;
                self.memory.program_counter = SR_7_ADDR;
                cycles = 3;
            },

            0xE9 => { // PCHL
                self.memory.program_counter = self.registers.hl_reg.get_pair();
            },

            // ****** Stack, IO, and Machine Control Group ******
            0x00 => (), // NOP
            0x76 => { // HLT
                match self.memory.stack_pointer.checked_sub(1) {
                    Some(x) => self.memory.stack_pointer = x,
                    None => return Err(CoreError::StackPointerOverflow)
                }
            },
            0xC1 => { // POP B
                self.registers.bc_reg.set_pair(self.memory.pop_stack()?);
                cycles = 3;
            },
            0xD1 => { // POP D
                self.registers.de_reg.set_pair(self.memory.pop_stack()?);
                cycles = 3;
            },
            0xE1 => { // POP H
                self.registers.hl_reg.set_pair(self.memory.pop_stack()?);
                cycles = 3;
            },
            0xF1 => { // POP PSW
                let data = self.memory.pop_stack()?; // Use local to avoid double reference
                self.restore_psw(data);
                cycles = 3;
            },
            0xC5 => { // PUSH B
                self.memory.push_stack(self.registers.bc_reg.get_pair())?;
                cycles = 3;
            },
            0xD5 => { // PUSH D
                self.memory.push_stack(self.registers.de_reg.get_pair())?;
                cycles = 3;
            },
            0xE5 => { // PUSH H
                self.memory.push_stack(self.registers.hl_reg.get_pair())?;
                cycles = 3;
            },
            0xF5 => { // PUSH PSW
                let data = self.generate_psw();
                self.memory.push_stack(data)?;
                cycles = 3;
            },
            0xE3 => { // XTHL
                let temp = self.memory.pop_stack()?;
                self.memory.push_stack(self.registers.hl_reg.get_pair())?;
                self.registers.hl_reg.set_pair(temp);
                cycles = 5;
            },
            0xF9 => { // SPHL
                self.memory.stack_pointer = self.registers.hl_reg.get_pair();
            },
            0xD3 => { // OUT d8
                let port = self.memory.fetch_byte()?;
                let data = self.registers.a_reg;
                match port {
                    0x02 => self.shifter.set_offset(data),
                    0x04 => self.shifter.load(data),
                    _ => ()
                }
                cycles = 3;
            },
            0xDB => { // IN d8
                let port = self.memory.fetch_byte()?;
                let data = match port {
                    0x01 => self.input.port1,
                    0x02 => self.input.port2,
                    0x03 => self.shifter.get_shift(),
                    _ => 0x00
                };
                self.registers.a_reg = data;
                cycles = 3;
            },
            0xF3 => { // DI
                self.interrupt_enable = false;
            },
            0xFB => { // EI
                self.interrupt_enable = true;
            },

            _ => {
                return Err(CoreError::OpcodeError { opcode: opcode })
                // panic!("Attempted to execute undefined instruction {:#04x}", opcode)
            }
        }

        Ok(cycles)
    }
}

/*
fn concat_bytes(a: u8, b: u8) -> u16 {
    (a as u16) << 8 & b as u16
}

fn split_bytes(a: u16) -> (u8, u8) {
    (((a & 0xFF00) >> 8) as u8, (a & 0x00FF) as u8)
}
*/

fn parity(a: u8) -> bool {
    // Shamelessly inspired by https://graphics.stanford.edu/~seander/bithacks.html#ParityParallel
    let mut a = a;
    a ^= a >> 4;
    a &= 0x0F;
    return !((0x6996 >> a) & 0x1 == 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const BAD_OPS:[u8; 13] = [0x08, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0xCB, 0xD9, 0xDD, 0xE3, 0xED, 0xFD];
    #[test]
    fn all_opcodes_exist() {
        let mut cpu = CPU::new();
        for op in 0x00..0xFF {
            if !BAD_OPS.contains(&op) {
                cpu.execute(op).unwrap();
            }
        }
    }
}