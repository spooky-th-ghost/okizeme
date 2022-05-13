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
    duration: u8
}

impl Freeze {
  pub fn new(duration: u8) -> Self {
    Freeze {duration}
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
      coms.entity(entity).remove::<Freeze>();
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
  MainMenu,
  InGame,
  PauseMenu,
}
