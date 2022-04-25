mod hitbox;
mod hurtbox;
use okizeme_types::PlayerId;

pub use hitbox::*;
pub use hurtbox::*;


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

impl Default for HitState {
  fn default() -> Self {
    Self::None
  }
}


pub struct CollisionEvent{
  pub collision: Collision,
  pub player_id: PlayerId,
  pub recieving_player_id: PlayerId,
}
