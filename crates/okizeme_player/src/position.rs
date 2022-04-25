use bevy::math::Vec3;
use okizeme_types::PlayerId;

pub struct Position {
  pub player_id: PlayerId,
  position: Vec3
}

impl Position {
  pub fn new(player_id: PlayerId, position: Vec3) -> Self {
    Position {
      player_id,
      position,
    }
  }

  pub fn set_position(&mut self, position: Vec3) {
    self.position = position;
  }

  pub fn get_position(&self) -> Vec3 {
    return self.position.clone();
  }
}
