use bevy::prelude::*;
use okizeme_utils::countdown;

/// Used to distinguish which player various game objects belong to
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum PlayerId {
  P1,
  P2
}

/// Primarily attached to enties when they should be skipped for animation 
/// and physics calculations (i.e. during hitpause or a super flash)
#[derive(Component)]
pub struct Freeze {
    duration: u8,
    stun_value: Option<u8>
}

impl Freeze {
  pub fn new(duration: u8, stun_value: Option<u8>) -> Self {
    Freeze {duration, stun_value}
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

pub fn manage_freeze(
  mut coms: Commands,
  mut query: Query<(Entity,&mut Freeze)>,
) {
  for  (entity, mut freeze) in query.iter_mut() {
    if freeze.is_finished() {
      if let Some(stun_frames) = freeze.stun_value {
         coms.entity(entity).remove::<Freeze>().insert(Stun::new(stun_frames));
      } else {
        coms.entity(entity).remove::<Freeze>();
      }
    }
  }
}

pub fn manage_stun(
  mut coms: Commands,
  mut query: Query<(Entity,&mut Stun)>,
) {
  for  (entity, mut hitstun) in query.iter_mut() {
    if hitstun.is_finished() {
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
