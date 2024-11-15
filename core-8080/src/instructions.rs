use crate::register_pair::RegisterPair;

pub fn stax(register: &mut RegisterPair, accumulator: &mut u8, memory: &mut [u8]) {
    memory[register.get_pair() as usize] = *accumulator;
}

pub fn inx(register: &mut RegisterPair) {
    register.set_pair(register.get_pair().wrapping_add(1));
}

mod tests {
    use super::*;
    #[test]
    fn op_inx() {
        let mut reg = RegisterPair {high: 0xFF, low: 0xFF};
        inx(&mut reg);
        assert_eq!(reg.get_pair(), 0);
    }
}