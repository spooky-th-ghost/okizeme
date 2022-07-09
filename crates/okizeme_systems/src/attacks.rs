use bevy::prelude::*;
use okizeme_types::Hitstop;
use okizeme_offense::AttackController;

pub fn handle_attacks (
    mut commands: Commands,
    mut query: Query<(Entity, &mut AttackController), Without<Hitstop>>
) {
    for (entity, mut attack_controller) in query.iter_mut() {
        // Get all (if any) attack events that should execute this frame
        if let Some(attack_events) = attack_controller.update() {
            //TODO: Spawn hitboxes from attack events here
        }

        if attack_controller.is_expired() {
            commands.entity(entity).remove::<AttackController>();
        }

    }
}
