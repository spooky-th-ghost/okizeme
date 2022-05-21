use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use okizeme_utils::*;

#[derive(Component, Clone, Debug, Default, Inspectable)]
pub struct Movement {
  pub jumpsquat: u8,
  pub air_jumps: u8,
  pub air_jumps_remaining: u8,
  pub airdashes: u8,
  pub airdashes_remaining: u8,
  pub air_dash_speed: f32,
  pub air_back_dash_speed: f32,
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
