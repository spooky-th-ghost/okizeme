use bevy::prelude::*;
use okizeme_utils::countdown;

mod config;

pub use config::*;
/// Used to distinguish which player various game objects belong to
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum PlayerId {
  P1,
  P2
}

pub trait SelfRemoving {
  fn countdown(&mut self);
  fn duration(&self) -> u8;

  fn is_finished(&mut self) -> bool {
    if self.duration() == 0 {
      true
    } else {
      self.countdown();
      false
    }
  }
}

macro_rules! SelfRemoving {
  (for $($t:ty),+) => {
      $(impl SelfRemoving for $t {
        fn duration(&self) -> u8 {
          self.0
        }
      
        fn countdown(&mut self) {
          self.0 = countdown(self.0);
        }
      })*
  }
}

/// Primarily attached to enties when they should be skipped for animation 
/// and physics calculations
#[derive(Component)]
pub struct Hitstop(pub u8);

//Component used to pause input reading and state updates while in block or hit stun
#[derive(Component)]
pub struct Stun(pub u8);

/// Primarily attached to enties when they should be skipped for animation 
/// and physics calculations
#[derive(Component)]
pub struct Busy(pub u8);


SelfRemoving!(for Hitstop, Stun, Busy);

pub fn manage_hitstop(
  mut coms: Commands,
  mut query: Query<(Entity,&mut Hitstop)>,
) {
  for  (entity, mut hitstop) in query.iter_mut() {
    if hitstop.is_finished() {
        coms.entity(entity).remove::<Hitstop>();
    }
  }
}

pub fn manage_stun(
  mut coms: Commands,
  mut query: Query<(Entity,&mut Stun), Without<Hitstop>>,
) {
  for  (entity, mut stun) in query.iter_mut() {
    if stun.is_finished() {
      coms.entity(entity).remove::<Stun>();
    }
  }
}

pub fn manage_busy(
  mut coms: Commands,
  mut query: Query<(Entity,&mut Busy), Without<Hitstop>>,
) {
  for  (entity, mut busy) in query.iter_mut() {
    if busy.is_finished() {
      coms.entity(entity).remove::<Busy>();
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
  MainMenu,
  InGame,
  PauseMenu,
}

#[derive(Debug, Clone, Copy)]
pub struct BusyEvent {
    pub player_id: PlayerId,
    pub busy_frames: u8
}
