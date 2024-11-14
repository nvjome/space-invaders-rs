use std::result;

const MEM_SIZE: usize = 0x10000;
const NUM_REGISTERS: usize = 6;

pub struct CPU {
    general_register: [u8; NUM_REGISTERS],
    memory: [u8; MEM_SIZE],
    program_counter: u16,
    stack: Vec<u16>,
    a_reg: u8, // accumulator
    b_reg: u8, // msb when paired with c
    c_reg: u8,
    d_reg: u8, // msb when paired with e
    e_reg: u8,
    h_reg: u8, // msb when paired with h
    l_reg: u8,
    condition_flags: ConditionFlags,
}

struct ConditionFlags {
    zero: bool,
    sign: bool,
    parity: bool,
    carry: bool,
    // aux_carry: bool, // Not used by Space Invades, so I'm ignoring it
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
                self.c_reg = self.fetch_next();
                self.b_reg = self.fetch_next();
            },
            0x02 => { // STAX B
                self.memory[concat_bytes(self.b_reg, self.c_reg) as usize] = self.a_reg;
            },
            0x03 => { // INX B
                (self.b_reg, self.c_reg) = split_bytes(concat_bytes(self.b_reg, self.c_reg).wrapping_add(1));
            },
            0x13 => { // INX D
                (self.d_reg, self.e_reg) = split_bytes(concat_bytes(self.d_reg, self.e_reg).wrapping_add(1));
            },
            0x04 => { // INR B
                self.b_reg = self.b_reg.wrapping_add(1);
                self.condition_flags.zero = self.b_reg == 0;
                self.condition_flags.sign = self.b_reg & 0x80 != 0;
                self.condition_flags.parity = parity(self.b_reg);
            },
            0x05 => { // DCR B
                let result = (self.b_reg);
            },
            _ => panic!("Attempted to execute undefined instruction {:#03x}", opcode)
        }
    }

    fn inx(&mut self) {
        todo!()
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