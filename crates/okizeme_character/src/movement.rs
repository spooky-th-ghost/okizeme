use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use okizeme_utils::*;

#[derive(Component, Clone, Debug, Inspectable)]
pub struct Movement {
  pub jumpsquat: u8,
  pub air_jumps: u8,
  pub air_jumps_remaining: u8,
  pub airdashes: u8,
  pub airdashes_remaining: u8,
  pub air_dash_speed: f32,
  pub air_back_dash_speed: f32,
  pub air_dash_lockout: u8,
  pub air_back_dash_lockout: u8,
  pub jump_lockout: u8,
  pub walk_speed: f32,
  pub back_walk_speed: f32,
  pub dash_speed: f32,
  pub gravity: f32,
  pub jump_height: f32,
  pub max_airdash_time: u8,
  pub max_air_backdash_time: u8,
  pub backdash: Backdash,
  pub facing_vector: f32,
  pub can_turn: bool,
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            jumpsquat: 3,
            air_jumps: 1,
            air_jumps_remaining: 1,
            airdashes: 1,
            airdashes_remaining: 1,
            air_dash_speed: 5.,
            air_back_dash_speed: 3.,
            air_dash_lockout: 5,
            air_back_dash_lockout: 2,
            jump_lockout: 8,
            walk_speed: 2.,
            back_walk_speed: 1.,
            dash_speed: 5.,
            gravity: 0.6,
            jump_height: 8.,
            max_airdash_time: 5,
            max_air_backdash_time: 2,
            backdash: Backdash::Standard { busy: 15, speed: 5., motion_duration: 15 },
            facing_vector: 1.,
            can_turn: true
        }
    }
}

impl Movement {
  pub fn can_airdash(&self) -> bool {
    self.airdashes_remaining > 0
  }

  pub fn spend_airdash(&mut self) {
    self.airdashes_remaining = countdown(self.airdashes_remaining);
  }

  pub fn land(&mut self) {
    self.air_jumps_remaining = self.air_jumps;
    self.airdashes_remaining = self.airdashes;
  }
}

#[derive(Clone, Copy, Debug, Inspectable)]
pub enum Backdash {
  Standard {busy: u8, speed: f32, motion_duration: u8},
  Teleport {busy: u8, distance: f32, motion_duration: u8},
  Leap {busy: u8, motion_duration: u8}
}

impl Default for Backdash {
  fn default() -> Self {
    Backdash::Standard{busy: 0, speed: 0.0, motion_duration: 0}
  }
}

impl Backdash {
  pub fn get_duration(&self) -> u8 {
    match self {
      Backdash::Standard {busy, speed: _, motion_duration: _} => *busy,
      Backdash::Teleport {busy, distance: _, motion_duration: _} => *busy,
      Backdash::Leap {busy, motion_duration: _} => *busy
    }
  }
}
