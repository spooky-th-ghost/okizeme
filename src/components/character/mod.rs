use bevy::prelude::*;

// #[derive(Component, PartialEq, Default, Debug, Reflect)]
// pub enum MoveState {
//     #[default]
//     Idle,
//     Walk,
//     Backwalk,
//     Airborne,
//     Crouching,
//     Dash,
//     DashBack,
//     Airdash,
//     AirdashBack,
//     HitStand,
//     HitAir,
//     Knockdown,
// }

// #[derive(Component, Default, Debug, Reflect)]
// pub struct Velocity {
//     pub direction: Vec2,
//     pub speed: f32,
//     pub weight: f32,
//     pub ground_collision: bool,
// }

// #[derive(Component, Default, Debug, Reflect)]
// pub struct Forces(pub Vec<Force>);

// #[derive(Default, Debug, Reflect, FromReflect)]
// pub struct Force {
//     pub duration: u8,
//     pub direction: Vec2,
//     pub speed: f32,
// }

// #[derive(Component, Default, Reflect)]
// pub struct Blocking {
//     pub duration: u8,
//     pub direction: BlockDirection,
//     pub barrier: bool,
// }

// #[derive(Default, Reflect)]
// pub enum BlockDirection {
//     #[default]
//     High,
//     Low,
//     Air,
// }

#[derive(Component, Default, Reflect)]
pub struct Jumpsquat {
    pub duration: u8,
    pub jump_velocity: Vec2,
}

#[derive(Component, Reflect)]
pub struct Grounded;

// #[derive(Component)]
// pub struct Counterhit {
//     duration: u8,
// }
