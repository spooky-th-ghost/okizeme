use bevy::{prelude::*, sprite::collide_aabb::collide};
use okizeme_defense::{BlockState, Hurtbox};
use okizeme_offense::{
    AttackController, CancelEvent, CancelTrigger, CollisionEvent, CollisionType, Hitbox,
    ImpactEvent,
};
use okizeme_types::{Busy, Hitstop, PlayerId, Stun};

pub fn cancel_hitboxes(
    mut coms: Commands,
    query: Query<(Entity, &PlayerId, &Hitbox)>,
    mut cancel_reader: EventReader<CancelEvent>,
) {
    for event in cancel_reader.iter() {
        for (entity, player_id, hitbox) in query.iter() {
            if *player_id == event.player_id {
                let mut remove_hitbox = || {
                    coms.entity(entity).despawn();
                };
                use CancelTrigger::*;
                match event.cancel_trigger {
                    Hit => {
                        remove_hitbox();
                    }
                    Block | Chain => {
                        if !hitbox.projectile() {
                            remove_hitbox();
                        }
                    }
                }
            }
        }
    }
}

pub fn cleanup_components_on_hit(
    mut commands: Commands,
    query: Query<(Entity, &PlayerId, AnyOf<(&Busy, &AttackController)>)>,
    mut impact_reader: EventReader<ImpactEvent>,
) {
    for event in impact_reader.iter() {
        for (entity, player_id, (busy, attack_controller)) in query.iter() {
            if &event.defense_id == player_id {
                if let Some(_) = busy {
                    commands.entity(entity).remove::<Busy>();
                }

                if let Some(_) = attack_controller {
                    commands.entity(entity).remove::<AttackController>();
                }
            }
        }
    }
}

pub fn detect_collisions(
    mut hitbox_query: Query<(&PlayerId, &mut Hitbox, &Sprite, &Transform)>,
    hurtbox_query: Query<(&PlayerId, &Sprite, &Transform), With<Hurtbox>>,
    mut collision_writer: EventWriter<CollisionEvent>,
) {
    for (hit_id, mut hitbox, hit_sprite, hit_transform) in hitbox_query.iter_mut() {
        for (hurt_id, hurt_sprite, hurt_transform) in hurtbox_query.iter() {
            if hitbox.active() && hit_id != hurt_id {
                let hit_pos = hit_transform.translation;
                let hit_size = if let Some(size) = hit_sprite.custom_size {
                    size
                } else {
                    Vec2::ZERO
                };

                let hurt_pos = hurt_transform.translation;
                let hurt_size = if let Some(size) = hurt_sprite.custom_size {
                    size
                } else {
                    Vec2::ZERO
                };

                if let Some(_collision) = collide(hit_pos, hit_size, hurt_pos, hurt_size) {
                    collision_writer.send(CollisionEvent {
                        hitbox: hitbox.clone(),
                        offense_id: *hit_id,
                        defense_id: *hurt_id,
                    });
                    hitbox.deactivate();
                }
            }
        }
    }
}

pub fn handle_collisions(
    mut commands: Commands,
    player_query: Query<(Entity, &PlayerId, &BlockState)>,
    mut collision_reader: EventReader<CollisionEvent>,
) {
    //TODO: Figure cancellation
    for event in collision_reader.iter() {
        let hitstop: u8 = event.hitbox.get_stun_value().hitstop;
        for (entity, player_id, block_state) in player_query.iter() {
            if !event.hitbox.projectile() {
                commands.entity(entity).insert(Hitstop(hitstop));
            }
            if *player_id == event.defense_id {
                let hit = event.hitbox.generate_collision(block_state);
                let stun_value = event.hitbox.get_stun_value();
                let stun_duration = match hit.collision_type {
                    CollisionType::StandHit { mixed: _ } => stun_value.standing_hitstun,
                    CollisionType::CrouchHit { mixed: _ } => stun_value.crouching_hitstun,
                    CollisionType::AirHit { mixed: _ } => stun_value.aerial_hitstun,
                    CollisionType::StandBlock { modifier }
                    | CollisionType::CrouchBlock { modifier }
                    | CollisionType::AirBlock { modifier } => {
                        modifier.get_stun_difference(stun_value.blockstun)
                    }
                };
                commands.entity(entity).insert(Stun(stun_duration));
                if event.hitbox.projectile() {
                    commands.entity(entity).insert(Hitstop(hitstop));
                }
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
