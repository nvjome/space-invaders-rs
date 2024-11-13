const MEM_SIZE: usize = 0x10000;
const NUM_REGISTERS: usize = 6;

pub struct CPU {
    general_register: [u8; NUM_REGISTERS],
    memory: [u8; MEM_SIZE],
    accumulator: u8,
    program_counter: u16,
    stack: Vec<u16>,
    b_reg: u8,
    c_reg: u8,
    d_reg: u8,
    e_reg: u8,
    h_reg: u8,
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
    pub fn cycle() {
        todo!();
    }
}