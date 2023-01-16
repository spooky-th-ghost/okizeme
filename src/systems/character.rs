use crate::*;
use bevy::prelude::*;

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
/// Manage and update ChracterState for all characters based on input
pub fn manage_character_state(
    mut player_positions: ResMut<PlayerPositions>,
    player_buffers: Res<PlayerInputSources>,
    mut query: Query<
        (&PlayerId, &mut CharacterState, &mut Movement, &mut Velocity),
        (Without<Hitstop>, Without<Busy>),
    >,
    mut transition_writer: EventWriter<AnimationTransitionEvent>,
) {
    for (player_id, mut state, mut movement, mut velocity) in query.iter_mut() {
        let position = player_positions.get_position(player_id);
        let buffer = player_buffers.get_source(player_id);
        //for buffer in player_data.buffers.iter_mut() {
        //if buffer.player_id == *player_id {
        let transition = state.update(buffer, &mut movement, &mut velocity, position);
        if let Some(t) = transition {
            if t == AnimationTransition::FallToIdle {
                movement.land();
            }
            //TODO add busy component here based on the current state transition
            transition_writer.send(AnimationTransitionEvent {
                player_id: *player_id,
                transition: t,
            });
        }
        //}
        //}
    }
}

///// New stuff

pub fn buffer_jumps(
    buffers: Res<PlayerInputSources>,
    // get the air jumps resources
    query: Query<
        (Entity, &PlayerId, Option<&Grounded>),
        (Without<Busy>, Without<Stun>, Without<Jumpsquat>),
    >,
) {
    for (entity, player_id, grounded) in &query {
        let buffer = buffers.get_source(player_id);
        // if buffer.current_action(Jump)
        // do jump here, probably just add jumpsquat
        // if grounded add grounded jumpsquat, else add aerial jumpsquat
        // if aerial, spend an airjump from the resource
    }
}

pub fn buffer_airdashes(
    buffers: Res<PlayerInputSources>,
    query: Query<
        (Entity, &PlayerId),
        (
            Without<Busy>,
            Without<Stun>,
            Without<Jumpsquat>,
            Without<Grounded>,
        ),
    >,
) {
    for (entity, player_id) in &query {
        let buffer = buffers.get_source(player_id);
        // if buffer.current_action(Airdash or Airbackdash)
        // buffer an airdash and add busy
        // spend the resource
    }
}
