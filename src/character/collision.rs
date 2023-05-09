use crate::types::{Frame, FrameRange, PlayerId};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct HitboxBundle {
    player_id: PlayerId,
    hitbox: Hitbox,
    sprite: Sprite,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
}

impl HitboxBundle {
    pub fn new(player_id: PlayerId, hitbox: Hitbox, position: Vec2, size: Vec2) -> Self {
        HitboxBundle {
            player_id,
            hitbox,
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        }
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct Hitbox {
    pub duration: Frame,
    pub lifetime: FrameRange,
    pub base_damage: u8,
}
#[derive(Clone, Default)]
pub struct HitboxEvent {
    pub frame: u8,
    pub position: Vec2,
    pub size: Vec2,
    pub hitbox: Hitbox,
}

pub struct Hurtbox {
    pub duration: Frame,
    pub lifetime: FrameRange,
}
#[derive(Clone, Default)]
pub struct HurtboxEvent {
    frame: Frame,
    head: Option<HurtboxProperties>,
    torso: Option<HurtboxProperties>,
    arms: Option<HurtboxProperties>,
    legs: Option<HurtboxProperties>,
}
#[derive(Clone)]
pub struct HurtboxProperties {
    position: Vec2,
    size: Vec2,
    invulnerability: bool,
}
