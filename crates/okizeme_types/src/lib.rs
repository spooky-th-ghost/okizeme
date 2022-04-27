use bevy::ecs::component::Component;

/// Used to distinguish which player various game objects belong to
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum PlayerId {
  P1,
  P2
}

/// Primarily attached to entities when a hitbox
/// connects to allow them to
/// opt out of animation and physics calculations
#[derive(Component)]
pub struct HitPause {
    pub duration: u8
}
