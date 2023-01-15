use bevy::prelude::*;

pub trait MovementResource {
    fn can_execute(&self) -> bool {
        self.remaining_uses() > 0
    }

    fn remaining_uses(&self) -> u8;

    fn refresh(&mut self);
}

#[derive(Resource)]
pub struct AirJumps {
    pub max_air_jumps: u8,
    pub air_jumps_remaining: u8,
}

impl MovementResource for AirJumps {
    fn remaining_uses(&self) -> u8 {
        self.air_jumps_remaining
    }

    fn refresh(&mut self) {
        self.air_jumps_remaining = self.max_air_jumps;
    }
}

#[derive(Resource)]
pub struct AirDashes {
    pub max_air_dashes: u8,
    pub air_dashes_remaining: u8,
}

impl MovementResource for AirDashes {
    fn remaining_uses(&self) -> u8 {
        self.air_dashes_remaining
    }

    fn refresh(&mut self) {
        self.air_dashes_remaining = self.max_air_dashes;
    }
}
