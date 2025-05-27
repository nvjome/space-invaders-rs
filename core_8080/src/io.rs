pub struct Inputs {
    pub port0: u8,
    pub port1: u8,
    pub port2: u8
}

pub enum ButtonState {Pressed, Released}

impl Default for Inputs {
    fn default() -> Self {
        Self::new()
    }
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            port0: 0x0E,
            port1: 0x08,
            port2: 0x08,
        }
    }

    pub fn player1_start(&mut self, state: ButtonState) {
        match state {
            ButtonState::Pressed => self.port1 |= 0x04,
            ButtonState::Released => self.port1 &= !0x04,
        }
    }

    pub fn player1_fire(&mut self, state: ButtonState) {
        match state {
            ButtonState::Pressed => self.port1 |= 0x10,
            ButtonState::Released => self.port1 &= !0x10,
        }
    }

    pub fn player1_left(&mut self, state: ButtonState) {
        match state {
            ButtonState::Pressed => self.port1 |= 0x20,
            ButtonState::Released => self.port1 &= !0x20,
        }
    }

    pub fn player1_right(&mut self, state: ButtonState) {
        match state {
            ButtonState::Pressed => self.port1 |= 0x40,
            ButtonState::Released => self.port1 &= !0x40,
        }
    }

    pub fn player2_start(&mut self, state: ButtonState) {
        match state {
            ButtonState::Pressed => self.port1 |= 0x02,
            ButtonState::Released => self.port1 &= !0x02,
        }
    }

    pub fn player2_fire(&mut self, state: ButtonState) {
        match state {
            ButtonState::Pressed => self.port2 |= 0x10,
            ButtonState::Released => self.port2 &= !0x10,
        }
    }

    pub fn player2_left(&mut self, state: ButtonState) {
        match state {
            ButtonState::Pressed => self.port2 |= 0x20,
            ButtonState::Released => self.port2 &= !0x20,
        }
    }

    pub fn player2_right(&mut self, state: ButtonState) {
        match state {
            ButtonState::Pressed => self.port2 |= 0x40,
            ButtonState::Released => self.port2 &= !0x40,
        }
    }
}