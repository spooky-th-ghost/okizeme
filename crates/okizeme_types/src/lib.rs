use bevy::prelude::*;
use okizeme_utils::countdown;

/// Used to distinguish which player various game objects belong to
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum PlayerId {
  P1,
  P2
}

/// Primarily attached to enties when they should be skipped for animation 
/// and physics calculations
#[derive(Component)]
pub struct Hitstop {
    duration: u8,
    stun_value: Option<u8>
}

impl Hitstop {
  pub fn new(duration: u8, stun_value: Option<u8>) -> Self {
    Hitstop {duration, stun_value}
  }
  pub fn is_finished(&mut self) -> bool {
    if self.duration == 0 {
      true
    } else {
      self.duration = countdown(self.duration);
      false
    }
  }
}

pub fn manage_hitstop(
  mut coms: Commands,
  mut query: Query<(Entity,&mut Hitstop)>,
) {
  for  (entity, mut hitstop) in query.iter_mut() {
    if hitstop.is_finished() {
      if let Some(stun_frames) = hitstop.stun_value {
        coms.entity(entity).remove::<Hitstop>().insert(Stun::new(stun_frames));
      } else {
        coms.entity(entity).remove::<Hitstop>();
      }
    }
  }
}

pub fn manage_stun(
  mut coms: Commands,
  mut query: Query<(Entity,&mut Stun)>,
) {
  for  (entity, mut stun) in query.iter_mut() {
    if stun.is_finished() {
      coms.entity(entity).remove::<Stun>();
    }
  }
}

#[derive(Component)]
pub struct Stun {
    duration: u8
}

impl Stun {
  pub fn new(duration: u8) -> Self {
    Stun {duration}
  }

  pub fn is_finished(&mut self) -> bool {
    if self.duration == 0 {
      true
    } else {
      self.duration = countdown(self.duration);
      false
    }
  }
}




#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
  MainMenu,
  InGame,
  PauseMenu,
}
