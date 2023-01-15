use crate::*;

pub struct Hit {
    pub hitbox: Hitbox,
    pub collision_type: CollisionType,
}

pub enum CollisionType {
    StandHit { mixed: bool },
    StandBlock { modifier: BlockModifier },
    CrouchHit { mixed: bool },
    CrouchBlock { modifier: BlockModifier },
    AirHit { mixed: bool },
    AirBlock { modifier: BlockModifier },
}

pub struct ImpactEvent {
    pub defense_id: PlayerId,
    pub offense_id: PlayerId,
    pub mixed: bool,
    pub hitbox: Hitbox,
}

pub struct CollisionEvent {
    pub hitbox: Hitbox,
    pub offense_id: PlayerId,
    pub defense_id: PlayerId,
}
