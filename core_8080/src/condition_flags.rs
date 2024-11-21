pub struct ConditionFlags {
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub carry: bool,
    // aux_carry: bool, // Not used by Space Invaders, so I'm ignoring it
}

impl ConditionFlags {
    pub fn new() -> Self {
        Self {
            zero: false,
            sign: false,
            parity: false,
            carry: false
        }
    }
}