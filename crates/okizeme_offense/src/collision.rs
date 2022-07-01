use okizeme_types::PlayerId;
use okizeme_defense::BlockModifier;
use crate::{
  Hitbox
};

pub struct Hit {
  pub hitbox: Hitbox,
  pub collision_type: CollisionType,
}

pub enum CollisionType {
    StandHit {mixed: bool},
    StandBlock {modifier: BlockModifier},
    CrouchHit {mixed: bool},
    CrouchBlock {modifier: BlockModifier},
    AirHit {mixed: bool},
    AirBlock {modifier: BlockModifier}
}

pub struct ImpactEvent {
    pub hit: Hit,
    pub defense_id: PlayerId,
    pub offense_id: PlayerId
}

pub struct CollisionEvent{
  pub hitbox: Hitbox,
  pub offense_id: PlayerId,
  pub defense_id: PlayerId,
}
