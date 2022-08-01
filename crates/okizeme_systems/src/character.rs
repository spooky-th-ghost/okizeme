use bevy::prelude::*;
use okizeme_animation::*;
use okizeme_character::{ActionState,  Movement};
use okizeme_physics::*;
use okizeme_resources::{PlayerInputSources, PlayerPositions};
use okizeme_types::{Busy, BusyEvent, Hitstop, PlayerId, Stun};

/// Manage and update ActionState for all characters based on input
pub fn manage_action_state(
    player_inputs: Res<PlayerInputSources>,
    mut query: Query<
        (&PlayerId, &mut ActionState, &mut Movement, &mut Velocity),
        (Without<Hitstop>, Without<Busy>, Without<Stun>),
    >,
    mut transition_writer: EventWriter<AnimationTransitionEvent>,
    mut busy_writer: EventWriter<BusyEvent>,
) {
    for (player_id, mut state, mut movement, mut velocity) in query.iter_mut() {
        let input_source = player_inputs.get_source(player_id);
        let (possible_transition, busy) = state.update(input_source, &mut movement, &mut velocity);
        if let Some(transition) = possible_transition {
            transition_writer.send(AnimationTransitionEvent {
                player_id: *player_id,
                transition,
            })
        }

        if busy > 0 {
            busy_writer.send(BusyEvent {
                player_id: *player_id,
                busy_frames: busy,
            })
        }
    }
}

pub fn add_busy(
    mut commands: Commands,
    query: Query<(Entity, &PlayerId)>,
    mut busy_reader: EventReader<BusyEvent>,
) {
    for event in busy_reader.iter() {
        for (entity, player_id) in query.iter() {
            if event.player_id == *player_id {
                commands.entity(entity).insert(Busy(event.busy_frames));
            }
        }
    }
}
