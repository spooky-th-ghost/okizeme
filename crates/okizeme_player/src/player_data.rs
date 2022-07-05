use bevy::prelude::*;
use okizeme_defense::CharacterHealth;
use okizeme_offense::{Combo, ComboedState, Hitbox};
use okizeme_input::{
    InputMap,
    RawButton,
    InputSource
};
use okizeme_types::PlayerId;
use crate::Position;

pub struct PlayerInputSources(Vec<InputSource>);
pub struct PlayerDevices(Vec<InputMap>);
pub struct PlayerPositions(Vec<Position>);
pub struct PlayerHealthBars(Vec<CharacterHealth>);
pub struct PlayerCombos(Vec<Combo>);

impl Default for PlayerDevices {
    fn default() -> Self {
        PlayerDevices(
            vec![
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
        )
    }
}

impl Default for PlayerInputSources {
    fn default() -> Self {
        PlayerInputSources(
            vec![
                InputSource::buffer(PlayerId::P1),
                InputSource::buffer(PlayerId::P2),
            ]
        )
    }
}

impl Default for PlayerPositions {
    fn default() -> Self {
        PlayerPositions(
            vec![
                Position::new(PlayerId::P1,Vec3::new(-50.0, 0.0, 0.0)),
                Position::new(PlayerId::P2,Vec3::new(50.0, 0.0, 0.0)),
            ]
        )
    }
}

impl Default for PlayerHealthBars {
    fn default() -> Self {
        PlayerHealthBars(
            vec![
                CharacterHealth::new(PlayerId::P1),
                CharacterHealth::new(PlayerId::P2)
            ]
        )
    }
}
impl Default for PlayerCombos {
    fn default() -> Self {
        PlayerCombos(Vec::new())
    }
}

fn get_player_index(player_id: &PlayerId) -> usize {
    match player_id {
      PlayerId::P1 => 0,
      PlayerId::P2 => 1,
    }
}

impl PlayerPositions {
  pub fn get_facing_vector(&self, player_id: &PlayerId) -> f32 {
    let p1_x_pos = self.0[0].get_position().x;
    let p2_x_pos = self.0[1].get_position().x;

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
    self.0[get_player_index(player_id)].set_position(position);
  }

  pub fn get_position(&mut self, player_id: &PlayerId) -> Vec3 {
    self.0[get_player_index(player_id)].get_position()
  }

  pub fn get_distance(&self) -> f32 {
    self.0[0].get_position().distance(self.0[1].get_position())
  }

  pub fn get_mid_point(&self) -> Vec2 {
    let p1 = self.0[0].get_position();
    let p2 = self.0[1].get_position();
    Vec2::new(p1.x+p2.x/2.0, p1.y+p2.y/2.0)
  }
}

impl PlayerInputSources {
    pub fn get_source_mut(&mut self, player_id: &PlayerId) -> &mut InputSource {
        &mut self.0[get_player_index(player_id)]
    }
    pub fn get_source(&self, player_id: &PlayerId) -> &InputSource {
        &self.0[get_player_index(player_id)]
    }
}

impl PlayerDevices {
    pub fn get(&self) -> &Vec<InputMap> {
        &self.0
    }
}
// impl PlayerData {
//   pub fn get_facing_vector(&self, player_id: &PlayerId) -> f32 {
//     let p1_x_pos = self.positions[0].get_position().x;
//     let p2_x_pos = self.positions[1].get_position().x;

//     if p1_x_pos > p2_x_pos {
//       if *player_id == PlayerId::P1 {
//         -1.0
//       } else {
//         1.0
//       }
//     } else if *player_id == PlayerId::P1 {
//       1.0
//     } else {
//       -1.0
//     }
//   }

//   pub fn set_position(&mut self, player_id: &PlayerId, position: Vec3) {
//     let i: usize = match player_id {
//       PlayerId::P1 => 0,
//       PlayerId::P2 => 1,
//     };
//     self.positions[i].set_position(position);
//   }

//   pub fn get_position(&mut self, player_id: &PlayerId) -> Vec3 {
//     let i: usize = match player_id {
//       PlayerId::P1 => 0,
//       PlayerId::P2 => 1,
//     };
//     self.positions[i].get_position()
//   }

//   pub fn get_distance(&self) -> f32 {
//     self.positions[0].get_position().distance(self.positions[1].get_position())
//   }

//   pub fn get_mid_point(&self) -> Vec2 {
//     let p1 = self.positions[0].get_position();
//     let p2 = self.positions[1].get_position();
//     Vec2::new(p1.x+p2.x/2.0, p1.y+p2.y/2.0)
//   }
// }

impl PlayerHealthBars {
    pub fn get_health(&self, player_id: &PlayerId) -> u16 {
        self.0[get_player_index(player_id)].current_value
    }

    pub fn get_health_percentage(&self, player_id: &PlayerId) -> f32 {
        self.0[get_player_index(player_id)].current_value as f32 / self.0[get_player_index(player_id)].max_value as f32
    }

    pub fn deal_damage(&mut self, player_id: &PlayerId, damage: u16) {
        self.0[get_player_index(player_id)].deal_damage(damage);
    }
}
impl PlayerCombos {
    pub fn add_to_combo(&mut self, hitbox: &Hitbox, player_id: &PlayerId, comboed_state: ComboedState, missed_tech: bool) -> (u16, u8) {
        let existing_combo: Option<&mut Combo> = self.0.iter_mut().find(|c| c.player_id == *player_id);
        if let Some(combo) = existing_combo {
            combo.add_to_combo(hitbox, missed_tech, comboed_state)
        } else {
            let mut new_combo = Combo::new(player_id);
            let (damage, hitstun) = new_combo.add_initial_hit_to_combo(hitbox, comboed_state);
            self.0.push(new_combo);
            (damage, hitstun)
        }
    }
}
