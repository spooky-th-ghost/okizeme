use crate::*;
use bevy::prelude::*;

mod config;

pub use config::*;
/// Used to distinguish which player various game objects belong to
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum PlayerId {
    P1,
    P2,
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
#[derive(Default, Debug, Clone, Copy, Component, Reflect)]
pub struct Hitstop(pub u8);

//Component used to pause input reading and state updates while in block or hit stun
#[derive(Default, Debug, Clone, Copy, Component, Reflect)]
pub struct Stun(pub u8);

/// Primarily attached to enties when they should be skipped for animation
/// and physics calculations
#[derive(Default, Debug, Clone, Copy, Component, Reflect)]
pub struct Busy(pub u8);

SelfRemoving!(for Hitstop, Stun, Busy);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    PauseMenu,
}

#[derive(Debug, Clone, Copy)]
pub struct BusyEvent {
    pub player_id: PlayerId,
    pub busy_frames: u8,
}
