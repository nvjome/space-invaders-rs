use crate::{register_pair::RegisterPair, CPU, ConditionFlagsStatus};

pub fn stax(register: &mut RegisterPair, accumulator: &mut u8, memory: &mut [u8]) {
    memory[register.get_pair() as usize] = *accumulator;
}

pub fn inx(register: &mut RegisterPair) {
    register.set_pair(register.get_pair().wrapping_add(1));
}

pub fn inr(register: &mut u8) -> ConditionFlagsStatus {
    *register = register.wrapping_add(1);
    let mut flags = ConditionFlagsStatus::new();
    flags.zero = Some(*register == 0);
    flags.sign = Some(*register & 0x80 != 0);
    flags.parity = Some(parity(*register));
    flags
}

fn parity(a: u8) -> bool {
    // Shamelessly inspired by https://graphics.stanford.edu/~seander/bithacks.html#ParityParallel
    let mut a = a;
    a ^= a >> 4;
    a &= 0x0F;
    return (0x6996 >> a) & 0x1 == 1;
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
