mod register_pair;
mod instructions;

use crate::register_pair::RegisterPair;

const MEM_SIZE: usize = 0x10000;

pub struct CPU {
    memory: [u8; MEM_SIZE],
    program_counter: u16,
    stack: Vec<u16>,
    a_reg: u8, // accumulator
    bc_reg: RegisterPair,
    de_reg: RegisterPair,
    hl_reg: RegisterPair,
    condition_flags: ConditionFlags,
}

struct ConditionFlags {
    zero: bool,
    sign: bool,
    parity: bool,
    carry: bool,
    // aux_carry: bool, // Not used by Space Invaders, so I'm ignoring it
}

struct ConditionFlagsStatus {
    zero: Option<bool>,
    sign: Option<bool>,
    parity: Option<bool>,
    carry: Option<bool>,
}

impl ConditionFlagsStatus {
    pub fn new() -> Self {
        ConditionFlagsStatus{
            zero: None,
            sign: None,
            parity: None,
            carry: None
        }
    }
}
/*
impl Default for ConditionFlagsStatus {
    fn default() -> Self {
        ConditionFlagsStatus::new()
    }
}
*/
enum AddessingMode {
    Direct,
    Register,
    RegisterPair,
    RegisterIndirect,
    RegisterPairIndirect,
    Immediate,
}

impl CPU {
    pub fn cycle(&mut self) {
        let opcode = self.fetch_byte();
        self.execute(opcode);
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory[self.program_counter as usize];
        self.program_counter += 1;
        byte
    }

    fn fetch_two_bytes(&mut self) -> u16 {
        let data_low = self.fetch_byte();
        let data_high = self.fetch_byte();
        (data_high as u16) << 8 & data_low as u16
    }

    fn process_condition_flags(&mut self, flags: ConditionFlagsStatus) {
        match flags.zero {
            Some(f) => self.condition_flags.zero = f,
            None => ()
        }
        match flags.sign {
            Some(f) => self.condition_flags.sign = f,
            None => ()
        }
        match flags.carry {
            Some(f) => self.condition_flags.carry = f,
            None => ()
        }
        match flags.parity {
            Some(f) => self.condition_flags.parity = f,
            None => ()
        }
    }

    fn execute(&mut self, opcode: u8) {
        // New condition flags s
        let mut new_flags = ConditionFlagsStatus {
            zero: None,
            sign: None,
            parity: None,
            carry: None
        };

        // Super big and ugly match statement because I'm not sure of a better way and I don't have
        match opcode {
            0x00 => (), // NOP
            0x01 => { // LXI B,d16
                let data = self.fetch_two_bytes();
                self.bc_reg.set_pair(data);
            },
            0x11 => { // LXI D,d16
                let data = self.fetch_two_bytes();
                self.de_reg.set_pair(data);
            },
            0x21 => { // LXI H,d16
                ;
            },
            0x02 => { // STAX B
                self.memory[self.bc_reg.get_pair() as usize] = self.a_reg;
                // instructions::stax(&mut self.bc_reg, &mut self.a_reg, &mut self.memory);
            },
            0x03 => { // INX B
                let result = self.bc_reg.get_pair().wrapping_add(1);
                self.bc_reg.set_pair(result);
            },
            0x13 => { // INX D
                let result = self.de_reg.get_pair().wrapping_add(1);
                self.de_reg.set_pair(result);
            },
            0x04 => { // INR B
                //self.b_reg = self.b_reg.wrapping_add(1);
                //self.condition_flags.zero = self.b_reg == 0;
                //self.condition_flags.sign = self.b_reg & 0x80 != 0;
                //self.condition_flags.parity = parity(self.b_reg);
                new_flags = instructions::inr(&mut self.bc_reg.high);
            },
            0x05 => { // DCR B
                //let result = (self.b_reg);
            },
            0x06 => { // MVI B,d8
                self.bc_reg.high = self.fetch_byte();
                },
            0x0E => { // MVI C,d8
                self.bc_reg.low = self.fetch_byte();
            }
            0x16 => { // MVI D,d8
                self.de_reg.high = self.fetch_byte();
                },
            0x1E => { // MVI E,d8
                self.de_reg.low = self.fetch_byte();
            },
            0x26 => { // MVI H,d8
                self.hl_reg.high = self.fetch_byte();
            },
            0x2E => { // MVI L,d8
                self.hl_reg.low = self.fetch_byte();
            },
            0x36 => { // MVI M,d8
                let addr = self.fetch_two_bytes();
                self.memory[addr as usize] = self.fetch_byte();
            },
            0x3E => { // MVI A,d8
                self.a_reg = self. fetch_byte();
            },
            0x22 => { // SHLD a16
                let addr = self.fetch_byte();
                self.memory[addr as usize] = self.hl_reg.high;
                self.memory[addr as usize + 1] = self.hl_reg.low;
            },
            _ => panic!("Attempted to execute undefined instruction {:#03x}", opcode)
        }

        self.process_condition_flags(new_flags);
    }
}

/*
fn concat_bytes(a: u8, b: u8) -> u16 {
    (a as u16) << 8 & b as u16
}

fn split_bytes(a: u16) -> (u8, u8) {
    (((a & 0xFF00) >> 8) as u8, (a & 0x00FF) as u8)
}

fn parity(a: u8) -> bool {
    // Shamelessly inspired by https://graphics.stanford.edu/~seander/bithacks.html#ParityParallel
    let mut a = a;
    a ^= a >> 4;
    a &= 0x0F;
    return (0x6996 >> a) & 0x1 == 1;
}
*/