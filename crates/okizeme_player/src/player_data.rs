use bevy::prelude::*;
use okizeme_input::{Buffer};
use okizeme_types::PlayerId;
use crate::{
  InputMap,
  Position,
  RawButton
};



pub struct PlayerData {
  pub local_devices: Vec<InputMap>,
  pub buffers: Vec<Buffer>,
  pub positions: Vec<Position>
}

impl Default for PlayerData {
  fn default() -> Self {
    PlayerData {
      local_devices: vec![
        InputMap {
            player_id: PlayerId::P1,
            a: RawButton::G(Gamepad(0),GamepadButtonType::West),
            b: RawButton::G(Gamepad(0),GamepadButtonType::North),
            c: RawButton::G(Gamepad(0),GamepadButtonType::RightTrigger),
            d: RawButton::G(Gamepad(0),GamepadButtonType::South),
            e: RawButton::G(Gamepad(0),GamepadButtonType::East),
            f: RawButton::G(Gamepad(0),GamepadButtonType::RightTrigger2),
            macro_1: RawButton::G(Gamepad(0),GamepadButtonType::LeftTrigger),
            macro_2: RawButton::G(Gamepad(0),GamepadButtonType::LeftTrigger2),
            x_positive: RawButton::G(Gamepad(0),GamepadButtonType::DPadRight),
            x_negative: RawButton::G(Gamepad(0),GamepadButtonType::DPadLeft),
            y_positive: RawButton::G(Gamepad(0),GamepadButtonType::DPadUp),
            y_negative: RawButton::G(Gamepad(0),GamepadButtonType::DPadDown),
        },
          InputMap {
            player_id: PlayerId::P2,
            a: RawButton::K(KeyCode::Y),
            b: RawButton::K(KeyCode::U),
            c: RawButton::K(KeyCode::I),
            d: RawButton::K(KeyCode::G),
            e: RawButton::K(KeyCode::H),
            f: RawButton::K(KeyCode::J),
            macro_1: RawButton::K(KeyCode::O),
            macro_2: RawButton::K(KeyCode::K),
            x_positive: RawButton::K(KeyCode::E),
            x_negative: RawButton::K(KeyCode::Q),
            y_positive: RawButton::K(KeyCode::Space),
            y_negative: RawButton::K(KeyCode::W),
        },
      ],
      buffers: vec![
        Buffer::new(PlayerId::P1),
        Buffer::new(PlayerId::P2),
      ],
      positions: vec![
        Position::new(PlayerId::P1,Vec3::new(-50.0, 0.0, 0.0)),
        Position::new(PlayerId::P2,Vec3::new(50.0, 0.0, 0.0)),
      ]
    }
  }
}

impl PlayerData {
  pub fn get_facing_vector(&self, player_id: &PlayerId) -> f32 {
    let p1_x_pos = self.positions[0].get_position().x;
    let p2_x_pos = self.positions[1].get_position().x;

    if p1_x_pos > p2_x_pos {
      if *player_id == PlayerId::P1 {
        -1.0
      } else {
        1.0
      }
    } else if *player_id == PlayerId::P1 {
      1.0
    } else {
      -1.0
    }
  }

  pub fn set_position(&mut self, player_id: &PlayerId, position: Vec3) {
    let i: usize = match player_id {
      PlayerId::P1 => 0,
      PlayerId::P2 => 1,
    };
    self.positions[i].set_position(position);
  }

  pub fn get_position(&mut self, player_id: &PlayerId) -> Vec3 {
    let i: usize = match player_id {
      PlayerId::P1 => 0,
      PlayerId::P2 => 1,
    };
    self.positions[i].get_position()
  }

  pub fn get_distance(&self) -> f32 {
    self.positions[0].get_position().distance(self.positions[1].get_position())
  }

  pub fn get_mid_point(&self) -> Vec2 {
    let p1 = self.positions[0].get_position();
    let p2 = self.positions[1].get_position();
    Vec2::new(p1.x+p2.x/2.0, p1.y+p2.y/2.0)
  }
}
