use bevy::{prelude::*, sprite::collide_aabb::collide};
use okizeme_defense::Hurtbox;
use okizeme_types::PlayerId;
use crate::{
  Hitbox,
  CancelEvent,
  CancelTrigger
};

pub fn cancel_hitboxes(
  mut coms: Commands,
  query: Query<(Entity, &PlayerId, &Hitbox)>,
  mut reader: EventReader<CancelEvent>
) {
  for event in reader.iter() {
    for (entity, player_id, hitbox) in query.iter() {
      if *player_id == event.player_id {
        let mut remove_hitbox = || {coms.entity(entity).despawn();};
        use CancelTrigger::*;
        match event.cancel_trigger {
          Hit => {
            remove_hitbox();
          },
          Block | Chain => {
            if !hitbox.is_projectile() {
              remove_hitbox();
            }
          },
        }
      }
    }
  }
}

pub fn detect_collisions(
  mut hitbox_query: Query<(&PlayerId, &mut Hitbox, &Sprite, &Transform)>,
  hurtbox_query: Query<(&PlayerId, &Sprite, &Transform), With<Hurtbox>>,
  //mut collision_writer: EventWriter
) {
  for (hit_id, mut hitbox, hit_sprite, hit_transform) in hitbox_query.iter_mut() {
    for (hurt_id, hurt_sprite, hurt_transform) in hurtbox_query.iter() {
      if hitbox.active && hit_id != hurt_id {
        let hit_pos = hit_transform.translation.clone();
        let hit_size = if let Some (size) = hit_sprite.custom_size {
          size
        } else {
          Vec2::ZERO
        };

        let hurt_pos = hurt_transform.translation.clone();
        let hurt_size = if let Some (size) = hurt_sprite.custom_size {
          size
        } else {
          Vec2::ZERO
        };

        if let Some(_collision) = collide(
          hit_pos,
          hit_size, 
          hurt_pos, hurt_size
        ) {
          // Send Collision event here
          hitbox.deactivate();
        }
      }
    }
  }
}
