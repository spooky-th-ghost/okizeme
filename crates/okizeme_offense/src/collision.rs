use okizeme_types::PlayerId;

use crate::{
  Hitbox
};

pub struct Hit {
  pub hitbox: Hitbox,
  pub blocked: bool,
}

impl Hit {
  pub fn new(hitbox: Hitbox, blocked: bool) -> Self {
    Hit {
      hitbox,
      blocked
    }
  }
}

pub struct HitEvent{
  pub hitbox: Hitbox,
  pub offense_id: PlayerId,
  pub defense_id: PlayerId,
}
