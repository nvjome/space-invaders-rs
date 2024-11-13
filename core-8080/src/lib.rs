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
    aux_carry: bool,
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
            0x01 => { // LXI rpBC,d16
                self.c_reg = self.fetch_next();
                self.b_reg = self.fetch_next();
            },
            _ => panic!("Attempted to execute undefined instruction {:#03x}", opcode)
        }
    }
}