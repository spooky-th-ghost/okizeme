use bevy::{
    prelude::*,
    animation::AnimationPlayer,
};

use okizeme_types::{Hitstop, PlayerId};

pub fn oki_animation_player(
    mut animation_players: Query<&mut AnimationPlayer,Without<Hitstop>>,
) {
    for  mut player in animation_players.iter_mut() {
        let elapsed = player.elapsed();
        player.set_elapsed( elapsed + (1./60.));
    }
}

pub struct AnimationController3D {

}

pub struct AnimationMap3D;

pub struct AnimationController2D {

}

pub struct AnimationMap2D;

/// Indicates if the [AnimationController] should loop it's current [Animation]
#[derive(Debug, PartialEq)]
pub enum AnimationState {
  LOOPING,
  SMEARING,
}

impl Default for AnimationState {
  fn default() -> Self {
    AnimationState::LOOPING
  }
}

/// Used to indicate what, if any, transition [Animation] should be played by the [AnimationController]
pub struct AnimationTransitionEvent {
  pub player_id: PlayerId,
  pub transition: AnimationTransition,
}

impl AnimationTransitionEvent {
  pub fn new(player_id: PlayerId, transition: AnimationTransition) -> Self {
    AnimationTransitionEvent {
      player_id,
      transition
    }
  }
}

/// Transition variants used by [AnimationTransitionEvent]
#[derive(Clone, PartialEq)]
pub enum AnimationTransition {
  WalkToIdle,
  BackwalkToIdle,
  CrouchToIdle,
  DashToIdle,
  BackDashToIdle,
  RiseToFall,
  FallToIdle,
  AirdashToFall,
  AirbackdashToFall,
  ToCrouch,
  ToWalk,
  ToBackwalk,
  ToDash,
  ToBackdash,
  ToRise,
  ToIdle,
  ToAirdash,
  ToAirBackdash,
  ToAttack {name: String},
}