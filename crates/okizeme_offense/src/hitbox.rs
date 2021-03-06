use crate::{AttackEvent, CollisionType, Hit};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use okizeme_defense::{BlockModifier, BlockState};
use okizeme_types::PlayerId;
use okizeme_utils::countdown;
use serde::{Deserialize, Serialize};

/// Box generated by attacks in game
#[derive(Component, Debug, Clone, Copy, Default, Inspectable)]
pub struct Hitbox {
    /// Base damage of the hitbox
    damage: u16,
    /// Proration when this hitbox connects first in a combo
    proration: f32,
    /// Force to be applied when a player is hit by this
    hit_force: Vec2,
    /// On block, how should this move the attacker
    attacker_block_force: Vec2,
    /// On block, how should this move the blocker
    defender_block_force: Vec2,
    /// Has the hitbox connected
    hit_state: HitState,
    /// Can the hitbox be blocked in the air
    air_blockable: bool,
    /// The block property of the hitbox
    property: AttackProperty,
    /// Strength of attack, used for hitstop/hitstun values
    level: AttackLevel,
    /// How many frames will this hitbox stay out
    duration: u8,
    /// Does this hitbox cause damage when blocked
    chip: bool,
    /// Is the hitbox currently active
    active: bool,
    /// Is the hitbox attached to the player that generated it
    projectile: bool,
}

impl Hitbox {
    // Mutations
    /// if possible, lower the hitboxes duration by 1 frame
    pub fn tick(&mut self) {
        self.duration = countdown(self.duration);
    }

    /// Set hitbox to inactive
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    // Getters
    /// Is the hitbox currently active?
    pub fn active(&self) -> bool {
        self.active
    }

    /// Is the hitbox a projectile
    pub fn projectile(&self) -> bool {
        self.projectile
    }

    /// Should the hitbox be removed
    pub fn finished(&self) -> bool {
        self.duration == 0
    }

    /// Base damage for the hitbox
    pub fn damage(&self) -> u16 {
        self.damage
    }

    /// Base proration for the hitbox
    pub fn proration(&self) -> f32 {
        self.proration
    }

    pub fn get_stun_value(&self) -> StunValue {
        StunValue::from_attack_level(self.level)
    }
    /// Attack Level
    pub fn level(&self) -> AttackLevel {
        self.level
    }

    pub fn generate_collision(&self, block_state: &BlockState) -> Hit {
        Hit {
            hitbox: *self,
            collision_type: self.get_collision_type(block_state),
        }
    }

    /// Returns if a Hitbox is blocked by a players {BlockState} whose {Hurtbox} it overlaps
    pub fn get_collision_type(&self, block_state: &BlockState) -> CollisionType {
        use BlockState::*;

        match block_state {
            Stand { barrier, instant } => {
                if self.property == AttackProperty::Low {
                    CollisionType::StandHit { mixed: true }
                } else {
                    let block_modifier = if *barrier && *instant {
                        BlockModifier::InstantBarrier
                    } else if !*barrier && *instant {
                        BlockModifier::Instant
                    } else if *barrier && !*instant {
                        BlockModifier::Barrier
                    } else {
                        BlockModifier::Normal
                    };
                    CollisionType::StandBlock {
                        modifier: block_modifier,
                    }
                }
            }
            Crouch { barrier, instant } => {
                if self.property == AttackProperty::High {
                    CollisionType::CrouchHit { mixed: true }
                } else {
                    let block_modifier = if *barrier && *instant {
                        BlockModifier::InstantBarrier
                    } else if !*barrier && *instant {
                        BlockModifier::Instant
                    } else if *barrier && !*instant {
                        BlockModifier::Barrier
                    } else {
                        BlockModifier::Normal
                    };
                    CollisionType::CrouchBlock {
                        modifier: block_modifier,
                    }
                }
            }
            Air { barrier, instant } => {
                if !self.air_blockable && !barrier && !instant {
                    CollisionType::AirHit { mixed: true }
                } else {
                    let block_modifier = if *barrier && *instant {
                        BlockModifier::InstantBarrier
                    } else if !*barrier && *instant {
                        BlockModifier::Instant
                    } else if *barrier && !*instant {
                        BlockModifier::Barrier
                    } else {
                        BlockModifier::Normal
                    };
                    CollisionType::AirBlock {
                        modifier: block_modifier,
                    }
                }
            } //self.air_blockable || *barrier || *instant,
            StandOpen => CollisionType::StandHit { mixed: false },
            CrouchOpen => CollisionType::CrouchHit { mixed: false },
            AirOpen => CollisionType::AirHit { mixed: false },
        }
    }
}

//#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Inspectable, PartialEq)]
pub enum AttackProperty {
    Mid,
    Low,
    High,
}

impl Default for AttackProperty {
    fn default() -> Self {
        AttackProperty::Mid
    }
}

//#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, Inspectable)]
pub enum HitState {
    None,
    Hit,
    Blocked,
}

impl Default for HitState {
    fn default() -> Self {
        HitState::None
    }
}

#[derive(Copy, Clone, Debug, Inspectable)]
pub enum AttackLevel {
    L0,
    L1,
    L2,
    L3,
    L4,
}

impl Default for AttackLevel {
    fn default() -> Self {
        AttackLevel::L0
    }
}

pub enum Knockdown {
    None,
    Stand,
    Soft,
    Mid,
    Hard,
}

pub struct StunValue {
    pub hitstop: u8,
    pub standing_hitstun: u8,
    pub crouching_hitstun: u8,
    pub aerial_hitstun: u8,
    pub blockstun: u8,
}

impl StunValue {
    fn new(
        hitstop: u8,
        standing_hitstun: u8,
        crouching_hitstun: u8,
        aerial_hitstun: u8,
        blockstun: u8,
    ) -> Self {
        StunValue {
            hitstop,
            standing_hitstun,
            crouching_hitstun,
            aerial_hitstun,
            blockstun,
        }
    }

    pub fn from_attack_level(attack_level: AttackLevel) -> Self {
        use AttackLevel::*;
        match attack_level {
            L0 => StunValue::new(11, 12, 13, 14, 9),
            L1 => StunValue::new(12, 14, 15, 16, 11),
            L2 => StunValue::new(13, 16, 17, 18, 13),
            L3 => StunValue::new(14, 19, 20, 21, 16),
            L4 => StunValue::new(15, 21, 22, 23, 18),
        }
    }
}

pub struct CancelEvent {
    pub player_id: PlayerId,
    pub cancel_trigger: CancelTrigger,
}

pub enum CancelTrigger {
    Chain,
    Hit,
    Block,
}

#[derive(Copy, Clone)]
pub enum ComboedState {
    Standing,
    Crouching,
    Juggle,
}
#[derive(Bundle)]
pub struct HitboxBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    player_id: PlayerId,
    hitbox: Hitbox,
}

pub trait SpawnHitbox {
    fn spawn_hitbox(
        &mut self,
        player_id: &PlayerId,
        attack_event: &AttackEvent,
        parent_transform: &Transform,
        facing_vector: f32,
        is_visible: bool,
    );
}

impl SpawnHitbox for Commands<'_, '_> {
    fn spawn_hitbox(
        &mut self,
        player_id: &PlayerId,
        attack_event: &AttackEvent,
        parent_transform: &Transform,
        facing_vector: f32,
        is_visible: bool,
    ) {
        let offset = Vec3::new(
            attack_event.location.x * facing_vector,
            attack_event.location.y,
            1.0,
        );
        let parent_translation = parent_transform.translation;
        let transform = Transform::from_translation(parent_translation + offset);

        self.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(attack_event.size),
                ..Default::default()
            },
            transform,
            visibility: Visibility { is_visible },
            ..Default::default()
        })
        .insert(*player_id)
        .insert(attack_event.hitbox);
    }
}
