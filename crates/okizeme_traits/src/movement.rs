
// Raw Traits
pub trait PositiveAirdash {
    fn execute_forward_air_dash(&mut self);
}

pub trait NegativeAirdash {
    fn execute_backward_air_dash(&self);
}

pub trait Airdash {
    fn get_remaining_air_dashes(&self);
    fn refresh_airdashes(&mut self);
}

pub trait PositiveDash {
    fn execute_forward_dash(&self);
}

pub trait NegativeDash {
    fn execute_backdash(&self);
}

pub trait MovementAction {
    fn get_busy(&self);
    fn get_motion_duration(&self);
}

pub trait AerialJump {
    fn execute_air_jump(&mut self);
    fn get_remaining_jumps(&self);
    fn refresh_jumps(&mut self);
}

// Composed Traits

pub trait ForwardAirdash: PositiveAirdash + Airdash + MovementAction {}
pub trait BackwardAirdash: NegativeAirdash + Airdash + MovementAction {}
pub trait Dash: PositiveDash + MovementAction {}
pub trait Backdash: NegativeDash + MovementAction {}
pub trait AirJump: AerialJump + MovementAction {}
