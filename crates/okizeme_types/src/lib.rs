use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use okizeme_utils::countdown;

mod config;

pub use config::*;
/// Used to distinguish which player various game objects belong to
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum PlayerId {
    P1,
    P2,
}

pub trait Timer {
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

macro_rules! Timer {
  (for $($t:ty),+) => {
      $(impl Timer for $t {
        fn duration(&self) -> u8 {
          self.0
        }

        fn countdown(&mut self) {
          self.0 = countdown(self.0);
        }
      })*
  }
}

pub enum TimerType {
    Hitstop,
    Stun,
    Busy,
    JumpLockout,
    AirdashLockout,
}
pub struct TimerEvent {
    pub player_id: PlayerId,
    pub duration: u8,
    pub timer_type: TimerType,
}

/// Primarily attached to enties when they should be skipped for animation
/// and physics calculations
#[derive(Component, Inspectable)]
pub struct Hitstop(pub u8);

/// Component used to pause input reading and state updates while in block or hit stun
#[derive(Component, Inspectable)]
pub struct Stun(pub u8);

/// Component used to pause input reading while a player is in a busy state
#[derive(Component, Inspectable)]
pub struct Busy(pub u8);

/// Component used to indicate when a player is unable to jump
#[derive(Component, Inspectable)]
pub struct JumpLockout(pub u8);

/// Component used to indicate when a player is unable to airdash
#[derive(Component, Inspectable)]
pub struct AirdashLockout(pub u8);

Timer!(for Hitstop, Stun, Busy, JumpLockout, AirdashLockout);

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
