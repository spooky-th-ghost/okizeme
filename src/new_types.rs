use crate::{Duration, PlayerId};
use bevy::prelude::{GlobalTransform, *};
use bevy::render::view::Visibility;
use bevy::sprite::Sprite;

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

#[derive(Component, Default)]
pub struct Hitbox {
    pub duration: Duration,
    pub base_damage: u8,
}
