use bevy::ecs::component::Component;

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
    pub duration: u8
}
