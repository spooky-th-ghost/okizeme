use okizeme_traits::*;
use okizeme_types::*;

mod backward_air_dash;
mod forward_air_dash;
mod jumps;

pub use backward_air_dash::*;
pub use forward_air_dash::*;
pub use jumps::*;

pub struct PlayerMovementData(Vec<Movement>);

impl Default for PlayerMovementData {
    fn default() -> Self {
        PlayerMovementData(vec![
            Movement::basic_movement(PlayerId::P1),
            Movement::basic_movement(PlayerId::P2),
        ])
    }
}

pub struct Movement {
    pub player_id: PlayerId,
    pub walk_speed: f32,
    pub back_walk_speed: f32,
    pub gravity: f32,
    pub jump: Jump,
    // pub air_jump: Box<dyn AirJump>,
    pub forward_air_dash: Box<dyn ForwardAirdash>,
    pub backward_air_dash: Box<dyn BackwardAirdash>,
    // pub forward_dash: Box<dyn Dash>,
    // pub backdash: Box<dyn Backdash>,
    // pub facing_vector: f32,
    // pub can_turn: bool,
}

//TODO: Finish basic movement, you'll need
// * BasicForwardAirdash
// * BasicBackwardAirdash
// * BasicAirJump
// * BasicDash
// * BasicBackDash
// Each with Defaults that make them match the example movement
impl Movement {
    pub fn basic_movement(player_id: PlayerId) -> Self {
        Movement {
            player_id,
            walk_speed: 2.,
            back_walk_speed: 1.,
            gravity: 0.6,
            jump: Jump::default(),
            forward_air_dash: Box::new(BasicForwardAirDash::default()),
            backward_air_dash: Box::new(BasicBackwardAirDash::default()),
        }
    }
}