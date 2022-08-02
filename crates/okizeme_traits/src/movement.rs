// Raw Traits
pub trait PositiveAirdash {
    fn execute_forward_air_dash(&mut self);
}

pub trait NegativeAirdash {
    fn execute_backward_air_dash(&mut self);
}

pub trait Airdash {
    fn get_remaining_air_dashes(&self) -> u8;
    fn refresh_airdashes(&mut self);
}

pub trait PositiveDash {
    fn execute_forward_dash(&self);
}

pub trait NegativeDash {
    fn execute_backdash(&self);
}

pub trait MovementAction {
    fn get_busy(&self) -> u8;
    fn get_motion_duration(&self) -> u8;
}

pub trait AerialJump {
    fn execute_air_jump(&mut self);
    fn get_remaining_jumps(&self);
    fn refresh_jumps(&mut self);
}

// Composed Traits

/// Required trait for a forward airdash, blanket implemented for any type implementing
/// [PositiveAirdash], [Airdash], and [MovementAction] like so:
/// ```
/// impl<T: PositiveAirdash + Airdash + MovementAction> ForwardAirdash for T {}
/// ```
pub trait ForwardAirdash: PositiveAirdash + Airdash + MovementAction {}
impl<T: PositiveAirdash + Airdash + MovementAction> ForwardAirdash for T {}

/// Required trait for a backwards airdash, blanket implemented for any type implementing
/// [NegativeAirdash], [Airdash], and [MovementAction] like so:
/// ```
/// impl<T: NegativeAirdash + Airdash + MovementAction> BackwardAirdash for T {}
/// ```
pub trait BackwardAirdash: NegativeAirdash + Airdash + MovementAction {}
impl<T: NegativeAirdash + Airdash + MovementAction> BackwardAirdash for T {}

/// Required trait for a forward ground dash, blanket implemented for any type implementing
/// [PositiveDash] and [MovementAction] like so:
/// ```
/// impl<T: PositiveDash + MovementAction> Dash for T {}
/// ```
pub trait Dash: PositiveDash + MovementAction {}
impl<T: PositiveDash + MovementAction> Dash for T {}

/// Required trat for all grounded backdashes, blanket implemented for any type implementing
/// [NegativeDash] and [MovementAction] like so:
/// ```
/// impl<T: NegativeDash + MovementAction> Backdash for T {}
/// ```
pub trait Backdash: NegativeDash + MovementAction {}
impl<T: NegativeDash + MovementAction> Backdash for T {}

/// Required trait for all air jumps, blanket implemented for any type implementing
/// [AerialJump] and [MovementAction]
/// ```
/// impl<T: AerialJump + MovementAction>
pub trait AirJump: AerialJump + MovementAction {}
