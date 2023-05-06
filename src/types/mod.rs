use crate::*;
use bevy::prelude::*;

mod config;

pub use config::*;
/// Used to distinguish which player various components belong to
#[derive(Default, Debug, Clone, Copy, PartialEq, Component, Reflect, FromReflect)]
pub enum PlayerId {
    #[default]
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

#[derive(Default, Clone, Copy)]
pub struct Frame(u8);
impl Frame {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn set(&mut self, value: u8) {
        self.0 = value;
    }
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}
#[derive(Default)]
pub struct Speed(pub f32);
impl Speed {
    pub fn get(&self) -> f32 {
        self.0
    }
    pub fn set(&mut self, value: f32) {
        self.0 = value;
    }
}
#[derive(Default)]
pub struct Distance(pub f32);
impl Distance {
    pub fn get(&self) -> f32 {
        self.0
    }
    pub fn set(&mut self, value: f32) {
        self.0 = value;
    }
}

#[derive(Default)]
pub struct Damage(f32);

#[derive(Default, Debug, Clone, Copy)]
pub struct FrameRange {
    start: u8,
    end: u8,
}

impl FrameRange {
    pub fn new(start: u8, end: u8) -> FrameRange {
        FrameRange { start, end }
    }

    pub fn to(end: u8) -> FrameRange {
        FrameRange { start: 0, end }
    }

    pub fn start(&self) -> u8 {
        self.start
    }

    pub fn end(&self) -> u8 {
        self.end
    }

    pub fn contains(&self, frame: Frame) -> bool {
        frame.get() <= self.end || frame.get() >= self.start
    }
}
