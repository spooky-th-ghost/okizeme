use bevy::prelude::*;

use okizeme_animation::*;
use okizeme_physics::*;
use okizeme_player::*;
use okizeme_types::{
  PlayerId,
  Hitstop,
  Busy
};

use crate::{
    CharacterState,
    Movement
};
/// Manage and update ChracterState for all characters based on input
pub fn manage_character_state(
    mut player_data: ResMut<PlayerData>,
    mut query: Query<(&PlayerId, &mut CharacterState, &mut Movement, &mut Velocity), (Without<Hitstop>, Without<Busy>)>,
    mut transition_writer: EventWriter<AnimationTransitionEvent>,
  ) {
    for (player_id, mut state, mut movement, mut velocity) in query.iter_mut() {
      let position = player_data.get_position(player_id);
      for buffer in player_data.buffers.iter_mut() {
        if buffer.player_id == *player_id {
          let transition = state.update(buffer,&mut movement, &mut velocity, position);
          if let Some(t) = transition {
              if t == AnimationTransition::FallToIdle {
                movement.land();
              }
            //TODO add busy component here based on the current state transition
            transition_writer.send(
        AnimationTransitionEvent {
                player_id: *player_id,
                transition: t,
              }
            );
          }
        }
      }
    }
  }
