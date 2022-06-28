use bevy::prelude::*;

#[derive(Component)]
pub struct Hurtbox;

#[derive(PartialEq, Clone, Copy, Component)]
pub enum BlockState {
  Stand {barrier: bool, instant: bool},
  Air {barrier: bool, instant: bool},
  Crouch {barrier: bool, instant: bool},
  StandOpen,
  CrouchOpen,
  AirOpen
}

#[derive(PartialEq, Clone, Copy)]
pub enum BlockModifier {
  Barrier,
  Instant,
  InstantBarrier,
  Normal
}

impl BlockModifier {
    pub fn get_stun_difference(&self, value: u8) -> u8 {
        use BlockModifier::*;
        match self {
            Barrier => value + 2,
            Instant => value - 2,
            InstantBarrier => value - 2,
            Normal => value
        }
    }
}
