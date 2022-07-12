use bevy::prelude::*;
use okizeme_types::PlayerId;
use okizeme_input::{
    InputMap,
    RawButton
};
pub struct PlayerDevices(Vec<InputMap>);

impl Default for PlayerDevices {
    fn default() -> Self {
        PlayerDevices(
            vec![
                InputMap {
                    player_id: PlayerId::P1,
                    a: RawButton::G(Gamepad(0),GamepadButtonType::West),
                    b: RawButton::G(Gamepad(0),GamepadButtonType::North),
                    c: RawButton::G(Gamepad(0),GamepadButtonType::RightTrigger),
                    d: RawButton::G(Gamepad(0),GamepadButtonType::South),
                    e: RawButton::G(Gamepad(0),GamepadButtonType::East),
                    f: RawButton::G(Gamepad(0),GamepadButtonType::RightTrigger2),
                    macro_1: RawButton::G(Gamepad(0),GamepadButtonType::LeftTrigger),
                    macro_2: RawButton::G(Gamepad(0),GamepadButtonType::LeftTrigger2),
                    x_positive: RawButton::G(Gamepad(0),GamepadButtonType::DPadRight),
                    x_negative: RawButton::G(Gamepad(0),GamepadButtonType::DPadLeft),
                    y_positive: RawButton::G(Gamepad(0),GamepadButtonType::DPadUp),
                    y_negative: RawButton::G(Gamepad(0),GamepadButtonType::DPadDown),
                },
                  InputMap {
                    player_id: PlayerId::P2,
                    a: RawButton::K(KeyCode::Y),
                    b: RawButton::K(KeyCode::U),
                    c: RawButton::K(KeyCode::I),
                    d: RawButton::K(KeyCode::G),
                    e: RawButton::K(KeyCode::H),
                    f: RawButton::K(KeyCode::J),
                    macro_1: RawButton::K(KeyCode::O),
                    macro_2: RawButton::K(KeyCode::K),
                    x_positive: RawButton::K(KeyCode::E),
                    x_negative: RawButton::K(KeyCode::Q),
                    y_positive: RawButton::K(KeyCode::Space),
                    y_negative: RawButton::K(KeyCode::W),
                },
            ],
        )
    }
}

impl PlayerDevices {
    pub fn get(&self) -> &Vec<InputMap> {
        &self.0
    }
}
