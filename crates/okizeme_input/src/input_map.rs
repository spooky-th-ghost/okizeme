use crate::{ButtonMask, Buttons};
use bevy::prelude::*;
use okizeme_types::PlayerId;
pub struct InputMap {
    pub player_id: PlayerId,
    pub a: RawButton,
    pub b: RawButton,
    pub c: RawButton,
    pub d: RawButton,
    pub e: RawButton,
    pub f: RawButton,
    pub g: RawButton,
    pub h: RawButton,
    pub x_positive: RawButton,
    pub x_negative: RawButton,
    pub y_positive: RawButton,
    pub y_negative: RawButton,
}

impl InputMap {
    pub fn get_raw_input_frame(
        &self,
        keyboard_input: &Res<Input<KeyCode>>,
        button_input: &Res<Input<GamepadButton>>,
    ) -> RawInputFrame {
        let right = match self.x_positive {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let left = match self.x_negative {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let down = match self.y_negative {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let up = match self.y_positive {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let a_pressed = match self.a {
            RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_pressed(GamepadButton(device_id, button_type))
            }
        };

        let b_pressed = match self.b {
            RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_pressed(GamepadButton(device_id, button_type))
            }
        };

        let c_pressed = match self.c {
            RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_pressed(GamepadButton(device_id, button_type))
            }
        };

        let d_pressed = match self.d {
            RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_pressed(GamepadButton(device_id, button_type))
            }
        };

        let e_pressed = match self.e {
            RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_pressed(GamepadButton(device_id, button_type))
            }
        };

        let f_pressed = match self.f {
            RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_pressed(GamepadButton(device_id, button_type))
            }
        };

        let g_pressed = match self.g {
            RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_pressed(GamepadButton(device_id, button_type))
            }
        };

        let h_pressed = match self.h {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let a_held = match self.a {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let b_held = match self.b {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let c_held = match self.c {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let d_held = match self.d {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let e_held = match self.e {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let f_held = match self.f {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let g_held = match self.g {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let h_held = match self.h {
            RawButton::K(keycode) => keyboard_input.pressed(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.pressed(GamepadButton(device_id, button_type))
            }
        };

        let a_released = match self.a {
            RawButton::K(keycode) => keyboard_input.just_released(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_released(GamepadButton(device_id, button_type))
            }
        };

        let b_released = match self.b {
            RawButton::K(keycode) => keyboard_input.just_released(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_released(GamepadButton(device_id, button_type))
            }
        };

        let c_released = match self.c {
            RawButton::K(keycode) => keyboard_input.just_released(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_released(GamepadButton(device_id, button_type))
            }
        };

        let d_released = match self.d {
            RawButton::K(keycode) => keyboard_input.just_released(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_released(GamepadButton(device_id, button_type))
            }
        };

        let e_released = match self.e {
            RawButton::K(keycode) => keyboard_input.just_released(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_released(GamepadButton(device_id, button_type))
            }
        };

        let f_released = match self.f {
            RawButton::K(keycode) => keyboard_input.just_released(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_released(GamepadButton(device_id, button_type))
            }
        };

        let g_released = match self.g {
            RawButton::K(keycode) => keyboard_input.just_released(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_released(GamepadButton(device_id, button_type))
            }
        };

        let h_released = match self.h {
            RawButton::K(keycode) => keyboard_input.just_released(keycode),
            RawButton::G(device_id, button_type) => {
                button_input.just_released(GamepadButton(device_id, button_type))
            }
        };

        let mut pressed_byte: u8 = 0b0000_0000;
        if a_pressed {
            pressed_byte |= 0b0000_0001
        }
        if b_pressed {
            pressed_byte |= 0b0000_0010
        }
        if c_pressed {
            pressed_byte |= 0b0000_0100
        }
        if d_pressed {
            pressed_byte |= 0b0000_1000
        }
        if e_pressed {
            pressed_byte |= 0b0001_0000
        }
        if f_pressed {
            pressed_byte |= 0b0010_0000
        }
        if g_pressed {
            pressed_byte |= 0b0100_0000
        }
        if h_pressed {
            pressed_byte |= 0b1000_0000
        }

        let mut released_byte: u8 = 0b0000_0000;
        if a_released {
            released_byte |= 0b0000_0001
        }
        if b_released {
            released_byte |= 0b0000_0010
        }
        if c_released {
            released_byte |= 0b0000_0100
        }
        if d_released {
            released_byte |= 0b0000_1000
        }
        if e_released {
            released_byte |= 0b0001_0000
        }
        if f_released {
            released_byte |= 0b0010_0000
        }
        if g_released {
            released_byte |= 0b0100_0000
        }
        if h_released {
            released_byte |= 0b1000_0000
        }

        let mut held_byte: u8 = 0b0000_0000;
        if a_held {
            held_byte |= 0b0000_0001
        }
        if b_held {
            held_byte |= 0b0000_0010
        }
        if c_held {
            held_byte |= 0b0000_0100
        }
        if d_held {
            held_byte |= 0b0000_1000
        }
        if e_held {
            held_byte |= 0b0001_0000
        }
        if f_held {
            held_byte |= 0b0010_0000
        }
        if g_held {
            held_byte |= 0b0100_0000
        }
        if h_held {
            held_byte |= 0b1000_0000
        }

        let buttons = Buttons {
            pressed: ButtonMask(pressed_byte),
            held: ButtonMask(held_byte),
            released: ButtonMask(released_byte),
        };

        RawInputFrame {
            buttons,
            right,
            left,
            up,
            down,
        }
    }
}

pub struct RawInputFrame {
    pub buttons: Buttons,
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
}

pub enum RawButton {
    K(KeyCode),
    G(Gamepad, GamepadButtonType),
}
