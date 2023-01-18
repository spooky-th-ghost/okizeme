use crate::*;

pub struct InputBuffer {
    pub player_id: PlayerId,
    pub motions: Vec<u8>,
    pub buttons: Vec<String>,
}
pub struct StaleButtons {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
    pub e: bool,
    pub f: bool,
    pub g: bool,
    pub h: bool,
}

#[derive(Default)]
pub struct ButtonStaleness {
    age: u8,
}

impl ButtonStaleness {
    pub fn is_stale(&self) -> bool {
        self.age == 5
    }

    pub fn age_button(&mut self) {
        if self.age < 5 {
            self.age += 1;
        }
    }

    pub fn reset_button_age(&mut self) {
        self.age = 0
    }
}
