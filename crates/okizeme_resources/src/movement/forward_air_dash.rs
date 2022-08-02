use okizeme_traits::*;

#[derive(Default)]
pub struct BasicForwardAirDash;

impl PositiveAirdash for BasicForwardAirDash {
    fn execute_forward_air_dash(&mut self) {}
}

impl Airdash for BasicForwardAirDash {
    fn get_remaining_air_dashes(&self) -> u8 {
        4
    }

    fn refresh_airdashes(&mut self) {}
}

impl MovementAction for BasicForwardAirDash {
    fn get_busy(&self) -> u8 {
        4
    }

    fn get_motion_duration(&self) -> u8 {
        4
    }
}
