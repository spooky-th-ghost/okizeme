use crate::*;
use bevy::prelude::*;

pub fn handle_attacks(
    mut commands: Commands,
    mut query: Query<(Entity, &PlayerId, &mut AttackController, &Transform), Without<Hitstop>>,
    config: Res<OkizemeConfig>,
    positions: Res<PlayerPositions>,
) {
    for (entity, player_id, mut attack_controller, transform) in query.iter_mut() {
        // Get all (if any) attack events that should execute this frame
        if let Some(attack_events) = attack_controller.update() {
            //TODO: Spawn hitboxes from attack events here
            let is_visible = config.get_hitbox_visibility(player_id);
            let facing_vector = positions.get_facing_vector(player_id);
            for attack_event in attack_events.iter() {
                commands.spawn_hitbox(
                    player_id,
                    attack_event,
                    transform,
                    facing_vector,
                    is_visible,
                )
            }
        }

        if attack_controller.is_expired() {
            commands.entity(entity).remove::<AttackController>();
        }
    }
}
