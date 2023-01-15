use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Blocking {
    pub duration: u8,
    pub direction: BlockDirection,
    pub barrier: bool,
}

#[derive(Default, Reflect)]
pub enum BlockDirection {
    #[default]
    High,
    Low,
    Air,
}

#[derive(Component, Default, Reflect)]
pub struct Jumpsquat {
    pub duration: u8,
    pub jump_velocity: Vec2,
}

#[derive(Component, Default, Reflect)]
pub struct Force {
    pub duration: u8,
    pub force: Vec2,
    pub gravity_frame: u8,
}
