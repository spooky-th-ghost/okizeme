use okizeme_types::PlayerId;

use crate::{
  Hitbox
};

pub struct Collision {
  pub hitbox: Hitbox,
  pub blocked: bool,
}

impl Collision {
  pub fn new(hitbox: Hitbox, blocked: bool) -> Self {
    Collision {
      hitbox,
      blocked
    }
  }
}

pub struct CollisionEvent{
  pub collision: Collision,
  pub offense_id: PlayerId,
  pub defense_id: PlayerId,
}
