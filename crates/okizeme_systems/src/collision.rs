
use bevy::{prelude::*, sprite::collide_aabb::collide};
use okizeme_defense::{
    Hurtbox, 
    BlockState
};
use okizeme_types::{
    PlayerId,
    Hitstop,
    Stun
};
use okizeme_offense::{
  Hitbox,
  CancelEvent,
  CancelTrigger, 
  CollisionEvent,
  HitEvent, 
  CollisionType
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
            if !hitbox.projectile() {
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
  mut collision_writer: EventWriter<CollisionEvent>
) {
  for (hit_id, mut hitbox, hit_sprite, hit_transform) in hitbox_query.iter_mut() {
    for (hurt_id, hurt_sprite, hurt_transform) in hurtbox_query.iter() {
      if hitbox.active() && hit_id != hurt_id {
        let hit_pos = hit_transform.translation;
        let hit_size = if let Some (size) = hit_sprite.custom_size {
          size
        } else {
          Vec2::ZERO
        };

        let hurt_pos = hurt_transform.translation;
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
          collision_writer.send(CollisionEvent {
            hitbox: hitbox.clone(),
            offense_id: *hit_id,
            defense_id: *hurt_id
          });
          hitbox.deactivate();
        }
      }
    }
  }
}

pub fn handle_collisions(
    player_query: Query<(&PlayerId, &BlockState)>,
    mut collision_reader: EventReader<CollisionEvent>,
    mut hit_writer: EventWriter<HitEvent>
) {
    for event in collision_reader.iter() {
        for (player_id, block_state) in player_query.iter() {
            if *player_id == event.defense_id {
                let hit = event.hitbox.generate_collision(block_state);
                hit_writer.send(HitEvent {
                   hit,
                   defense_id: event.defense_id,
                   offense_id: event.offense_id
                })
            }
        }
    }
}

pub fn handle_hits(
    mut commands: Commands,
    player_query: Query<(Entity, &PlayerId)>,
    mut hit_reader: EventReader<HitEvent>
) {
    for event in hit_reader.iter() {
        let stun_value = event.hit.hitbox.get_stun_value();
        for (entity, player_id) in player_query.iter() {
            commands.entity(entity).insert(Hitstop::new(stun_value.hitstop));
            if *player_id == event.defense_id {
                let stun_duration = match event.hit.collision_type {
                   CollisionType::StandHit {mixed:_} => stun_value.standing_hitstun,
                   CollisionType::CrouchHit { mixed:_} => stun_value.crouching_hitstun,
                   CollisionType::AirHit { mixed:_} => stun_value.aerial_hitstun,
                   CollisionType::StandBlock {modifier}
                   | CollisionType::CrouchBlock {modifier}
                   | CollisionType::AirBlock {modifier} => modifier.get_stun_difference(stun_value.blockstun),
                };
                commands.entity(entity).insert(Stun::new(stun_duration));
            }
        }
    }
}
  // TODO: Handle collisions here
  // Essentially, need to grab the following things:
  //  Commands:
  //  - Used to apply hitstop components
  //  Components:
  // - Player Id (To find who to apply stun, damage and hitstop to)
  // - CharacterState (To determine state and whether the hit can be blocked)
  // - Velocity
  // Resources:
  // - Inputs (Part of determining if the hitbox is blocked)
  // - Health (yet created, to apply damage)
  // - Combo (if a hit, add to the attackers combo and use it to determine damage/hitstun values, else start a combo)
  // Events:
  // - Hit (Reader: Find any collisions generated this frame)
  // - Transition (Writer: Transition the collision reciever to a blocking or hit state)
