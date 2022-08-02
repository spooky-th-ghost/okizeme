use okizeme_traits::*;

#[derive(Default)]
pub struct BasicBackwardAirDash;

impl NegativeAirdash for BasicBackwardAirDash {
    fn execute_backward_air_dash(&mut self) {}
}

impl Airdash for BasicBackwardAirDash {
    fn get_remaining_air_dashes(&self) -> u8 {
        4
    }

    fn refresh_airdashes(&mut self) {}
}

impl MovementAction for BasicBackwardAirDash {
    fn get_busy(&self) -> u8 {
        4
    }

    fn get_motion_duration(&self) -> u8 {
        4
    }
}
