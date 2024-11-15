mod register_pair;
mod instructions;

use instructions::stax;

use crate::register_pair::RegisterPair;

const MEM_SIZE: usize = 0x10000;
const NUM_REGISTERS: usize = 6;

pub struct CPU {
    general_register: [u8; NUM_REGISTERS],
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
        let opcode = self.fetch_next();
        self.execute(opcode);
    }

    fn fetch_next(&mut self) -> u8 {
        let byte = self.memory[self.program_counter as usize];
        self.program_counter += 1;
        byte
    }

    fn execute(&mut self, opcode: u8) {
        // Super big and ugly match statement because I'm not sure of a better way and I don't have
        match opcode {
            0x00 => (), // NOP
            0x01 => { // LXI B,d16
                self.bc_reg.low = self.fetch_next();
                self.bc_reg.high = self.fetch_next();
            },
            0x02 => { // STAX B
                // self.memory[self.bc_reg.get_pair() as usize] = self.a_reg;
                instructions::stax(&mut self.bc_reg, &mut self.a_reg, &mut self.memory);
            },
            0x03 => { // INX B
                // (self.bc_reg.high, self.bc_reg.low) = split_bytes(concat_bytes(self.bc_reg.high, self.bc_reg.low).wrapping_add(1));
                // self.bc_reg.set_pair(self.bc_reg.get_pair().wrapping_add(1));
                instructions::inx(&mut self.bc_reg);
            },
            0x13 => { // INX D
                // (self.d_reg, self.e_reg) = split_bytes(concat_bytes(self.d_reg, self.e_reg).wrapping_add(1));
                // self.de_reg.set_pair(self.de_reg.get_pair().wrapping_add(1));
                instructions::inx(&mut self.de_reg);
            },
            0x04 => { // INR B
                //self.b_reg = self.b_reg.wrapping_add(1);
                //self.condition_flags.zero = self.b_reg == 0;
                //self.condition_flags.sign = self.b_reg & 0x80 != 0;
                //self.condition_flags.parity = parity(self.b_reg);
            },
            0x05 => { // DCR B
                //let result = (self.b_reg);
            },
            _ => panic!("Attempted to execute undefined instruction {:#03x}", opcode)
        }
    }
}

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