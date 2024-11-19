mod registers;
mod memory;

use crate::registers::Registers;
use crate::memory::Memory;

pub struct CPU {
    memory: Memory,
    stack: Vec<u16>,
    registers: Registers,
    flags: ConditionFlags,
}

struct ConditionFlags {
    zero: bool,
    sign: bool,
    parity: bool,
    carry: bool,
    // aux_carry: bool, // Not used by Space Invaders, so I'm ignoring it
}

/*struct ConditionFlagsStatus {
    zero: Option<bool>,
    sign: Option<bool>,
    parity: Option<bool>,
    carry: Option<bool>,
}

enum AddessingMode {
    Direct,
    Register,
    RegisterPair,
    RegisterIndirect,
    RegisterPairIndirect,
    Immediate,
}*/

impl CPU {
    pub fn cycle(&mut self) {
        let opcode = self.memory.fetch_byte();
        self.execute(opcode);
    }

    /*fn process_condition_flags(&mut self, flags: ConditionFlagsStatus) {
        match flags.zero {
            Some(f) => self.flags.zero = f,
            None => ()
        }
        match flags.sign {
            Some(f) => self.flags.sign = f,
            None => ()
        }
        match flags.carry {
            Some(f) => self.flags.carry = f,
            None => ()
        }
        match flags.parity {
            Some(f) => self.flags.parity = f,
            None => ()
        }
    }
    */

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

    fn execute(&mut self, opcode: u8) {
        // Super big and ugly match statement because I'm not sure of a better way and I don't have
        match opcode {
            0x00 => (), // NOP

            // ****** Data Transfer Group ******
            0x01 => { // LXI B,d16
                self.registers.bc_reg.set_pair(self.memory.fetch_two_bytes());
            },
            0x11 => { // LXI D,d16
                self.registers.de_reg.set_pair(self.memory.fetch_two_bytes());
            },
            0x21 => { // LXI H,d16
                self.registers.hl_reg.set_pair(self.memory.fetch_two_bytes());
            },
            0x31 => { // LXI SP,d16
                todo!()
            },
            0x02 => { // STAX B
                self.memory.save_byte(self.registers.bc_reg.get_pair(), self.registers.a_reg);
            },
            0x03 => { // INX B
                self.registers.bc_reg.set_pair(self.registers.bc_reg.get_pair().wrapping_add(1));
            },
            0x13 => { // INX D
                self.registers.de_reg.set_pair(self.registers.de_reg.get_pair().wrapping_add(1));
            },
            0x04 => { // INR B
                self.registers.bc_reg.high = self.registers.bc_reg.high.wrapping_add(1);
                self.flags.zero = self.registers.bc_reg.high == 0;
                self.flags.sign = self.registers.bc_reg.high & 0x80 != 0;
                self.flags.parity = parity(self.registers.bc_reg.high);
            },
            0x05 => { // DCR B
                todo!()
            },
            0x06 => { // MVI B,d8
                self.registers.bc_reg.high = self.memory.fetch_byte();
                },
            0x0E => { // MVI C,d8
                self.registers.bc_reg.low = self.memory.fetch_byte();
            }
            0x16 => { // MVI D,d8
                self.registers.de_reg.high = self.memory.fetch_byte();
                },
            0x1E => { // MVI E,d8
                self.registers.de_reg.low = self.memory.fetch_byte();
            },
            0x26 => { // MVI H,d8
                self.registers.hl_reg.high = self.memory.fetch_byte();
            },
            0x2E => { // MVI L,d8
                self.registers.hl_reg.low = self.memory.fetch_byte();
            },
            0x36 => { // MVI M,d8
                let data = self.memory.fetch_byte();
                let addr = self.registers.hl_reg.get_pair();
                self.memory.save_byte(addr, data);
            },
            0x3E => { // MVI A,d8
                self.registers.a_reg = self.memory.fetch_byte();
            },
            0x22 => { // SHLD a16
                let addr = self.memory.fetch_two_bytes();
                self.memory.save_two_bytes(addr, self.registers.hl_reg.get_pair());
            },

            // ****** Arithmetic Group ******

            // ****** Branch Group ******
            // *** Returns ***
            0xC9 => { // RET
                self.memory.program_counter = self.stack.pop().unwrap(); // Pop program counter from the stack
            },
            0xC0 => { // RNZ
                if self.flags.zero == false {
                    self.memory.program_counter = self.stack.pop().unwrap();
                }
            },
            0xC8 => { // RZ
                if self.flags.zero == true {
                    self.memory.program_counter = self.stack.pop().unwrap();
                }
            },
            0xD0 => { // RNC
                if self.flags.carry == false {
                    self.memory.program_counter = self.stack.pop().unwrap();
                }
            },
            0xD8 => { // RC
                if self.flags.carry == true {
                    self.memory.program_counter = self.stack.pop().unwrap();
                }
            },
            0xE0 => { // RPO
                if self.flags.parity == false {
                    self.memory.program_counter = self.stack.pop().unwrap();
                }
            },
            0xE8 => { // RPE
                if self.flags.parity == true {
                    self.memory.program_counter = self.stack.pop().unwrap();
                }
            },
            0xF0 => { // RP
                if self.flags.sign == false {
                    self.memory.program_counter = self.stack.pop().unwrap();
                }
            },
            0xF8 => { // RM
                if self.flags.sign == true {
                    self.memory.program_counter = self.stack.pop().unwrap();
                }
            },

            // *** Calls ***
            0xCD => { // CALL a16
                self.stack.push(self.memory.program_counter); // Push program counter to the stack
                self.memory.program_counter = self.memory.fetch_two_bytes(); // Set program counter to new address
            }
            
            // *** Jumps ***
            0xC3 => { //JMP a16
                self.memory.program_counter = self.memory.fetch_two_bytes();
            },
            0xC2 => { // JNZ a16
                if self.flags.zero == false {
                    self.memory.program_counter = self.memory.fetch_two_bytes();
                }
            },
            0xCA => { // JZ a16
                if self.flags.zero == true {
                    self.memory.program_counter = self.memory.fetch_two_bytes();
                }
            },
            0xD2 => { // JNC a16
                if self.flags.carry == false {
                    self.memory.program_counter = self.memory.fetch_two_bytes();
                }
            },
            0xDA => { // JC a16
                if self.flags.carry == true {
                    self.memory.program_counter = self.memory.fetch_two_bytes();
                }
            },
            0xE2 => { // JPO a16
                if self.flags.parity == false {
                    self.memory.program_counter = self.memory.fetch_two_bytes();
                }
            },
            0xEA => { // JPE a16
                if self.flags.parity == true {
                    self.memory.program_counter = self.memory.fetch_two_bytes();
                }
            },
            0xF2 => { // JP a16
                if self.flags.sign == false {
                    self.memory.program_counter = self.memory.fetch_two_bytes();
                }
            },
            0xFA => { // JM a16
                if self.flags.sign == true {
                    self.memory.program_counter = self.memory.fetch_two_bytes();
                }
            },

            // ****** Stack, IO, and Machine Control Group ******
            0xC1 => { // POP B
                self.registers.bc_reg.set_pair(self.stack.pop().unwrap());
            },
            0xD1 => { // POP D
                self.registers.de_reg.set_pair(self.stack.pop().unwrap());
            },
            0xE1 => { // POP H
                self.registers.hl_reg.set_pair(self.stack.pop().unwrap());
            },
            0xF1 => { // POP PSW
                let data = self.stack.pop().unwrap(); // Use local avoid double reference
                self.restore_psw(data);
            },
            0xC5 => { // PUSH B
                self.stack.push(self.registers.bc_reg.get_pair());
            },
            0xD5 => { // PUSH D
                self.stack.push(self.registers.de_reg.get_pair());
            },
            0xE5 => { // PUSH H
                self.stack.push(self.registers.hl_reg.get_pair());
            },
            0xF5 => { // PUSH PSW
                let data = self.generate_psw();
                self.stack.push(data);
            },

            _ => panic!("Attempted to execute undefined instruction {:#03x}", opcode)
        }
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
    return (0x6996 >> a) & 0x1 == 1;
}
