use bevy::prelude::*;
use okizeme_utils::countdown;

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
          self.duration
        }
      
        fn countdown(&mut self) {
          self.duration = countdown(self.duration);
        }
      })*
  }
}

/// Primarily attached to enties when they should be skipped for animation 
/// and physics calculations
#[derive(Component)]
pub struct Hitstop {
    duration: u8
}

impl Hitstop {
    pub fn new(duration: u8) -> Self {
        Hitstop {duration}
    }
}

//Component used to pause input reading and state updates while in block or hit stun
#[derive(Component)]
pub struct Stun {
    duration: u8
}

impl Stun{
    pub fn new(duration: u8) -> Self {
        Stun {duration}
    }
}

/// Primarily attached to enties when they should be skipped for animation 
/// and physics calculations
#[derive(Component)]
pub struct Busy{
  duration: u8
}

impl Busy {
  pub fn new(duration: u8) -> Self {
    Busy {duration}
  }
}

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
  mut query: Query<(Entity,&mut Busy)>,
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
