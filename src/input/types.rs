use bevy::reflect::{FromReflect, Reflect};
use std::fmt;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub enum CommandMotion {
    Direction(MotionMask),
    #[default]
    Dash,
    Backdash,
    Qcf,
    Qcb,
    Dp,
    Rdp,
    TwoTwo,
    DoubleQcf,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct CommandInput {
    motion: CommandMotion,
    button: ButtonMask,
}

impl CommandInput {
    pub fn new(motion: CommandMotion, button_str: &str) -> Self {
        CommandInput {
            motion,
            button: ButtonMask::with_buttons(button_str),
        }
    }

    pub fn motion(&self) -> CommandMotion {
        self.motion
    }

    pub fn button(&self) -> ButtonMask {
        self.button
    }

    fn raw(motion: CommandMotion, button: ButtonMask) -> Self {
        CommandInput { motion, button }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct InputTree {
    motion_commands: Vec<CommandInput>,
    buffered_motion: MotionMask,
    buffered_button: ButtonMask,
}

impl InputTree {
    pub fn from_input(motions: &str, buttons: ButtonStream, last_motion: MotionMask) -> InputTree {
        use crate::input::{motion_parsing::*, Parser};

        let mut motions_vec = Vec::new();
        let dqcf = double_qcf();
        let dp = one_or_more(dp());
        let rdp = one_or_more(rdp());
        let qcf = one_or_more(qcf());
        let qcb = one_or_more(qcb());
        let two_two = one_or_more(two_two());
        let dash = one_or_more(dash());
        let backdash = one_or_more(backdash());

        match dqcf.parse(motions) {
            Ok((remaining, motion)) => {
                motions_vec.push(CommandInput::raw(
                    motion,
                    buttons.buffered_at_index(motions.len() - remaining.len()),
                ));
            }
            _ => (),
        }
        match dp.parse(motions) {
            Ok((remaining, mut found_motions)) => {
                if let Some(motion) = found_motions.pop() {
                    motions_vec.push(CommandInput::raw(
                        motion,
                        buttons.buffered_at_index(motions.len() - remaining.len()),
                    ));
                }
            }
            _ => (),
        }
        match rdp.parse(motions) {
            Ok((remaining, mut found_motions)) => {
                if let Some(motion) = found_motions.pop() {
                    motions_vec.push(CommandInput::raw(
                        motion,
                        buttons.buffered_at_index(motions.len() - remaining.len()),
                    ));
                }
            }
            _ => (),
        }
        match qcf.parse(motions) {
            Ok((remaining, mut found_motions)) => {
                if let Some(motion) = found_motions.pop() {
                    motions_vec.push(CommandInput::raw(
                        motion,
                        buttons.buffered_at_index(motions.len() - remaining.len()),
                    ));
                }
            }
            _ => (),
        }
        match qcb.parse(motions) {
            Ok((remaining, mut found_motions)) => {
                if let Some(motion) = found_motions.pop() {
                    motions_vec.push(CommandInput::raw(
                        motion,
                        buttons.buffered_at_index(motions.len() - remaining.len()),
                    ));
                }
            }
            _ => (),
        }
        match two_two.parse(motions) {
            Ok((remaining, mut found_motions)) => {
                if let Some(motion) = found_motions.pop() {
                    motions_vec.push(CommandInput::raw(
                        motion,
                        buttons.buffered_at_index(motions.len() - remaining.len()),
                    ));
                }
            }
            _ => (),
        }
        match dash.parse(motions) {
            Ok((remaining, mut found_motions)) => {
                if let Some(motion) = found_motions.pop() {
                    motions_vec.push(CommandInput::raw(
                        motion,
                        buttons.buffered_at_index(motions.len() - remaining.len()),
                    ));
                }
            }
            _ => (),
        }
        match backdash.parse(motions) {
            Ok((remaining, mut found_motions)) => {
                if let Some(motion) = found_motions.pop() {
                    motions_vec.push(CommandInput::raw(
                        motion,
                        buttons.buffered_at_index(motions.len() - remaining.len()),
                    ));
                }
            }
            _ => (),
        }

        InputTree {
            motion_commands: motions_vec,
            buffered_motion: last_motion,
            buffered_button: buttons.buffered(),
        }
    }

    pub fn command_inputs(&self) -> Vec<CommandInput> {
        let mut inputs = self.motion_commands.clone();
        inputs.sort_by(|a, b| a.motion().partial_cmp(&b.motion()).unwrap());
        inputs
    }

    pub fn is_dashing(&self) -> Option<DashType> {
        if let Some(dash) = self
            .motion_commands
            .iter()
            .find(|m| matches!(m.motion(), CommandMotion::Dash | CommandMotion::Backdash))
        {
            match dash.motion() {
                CommandMotion::Dash => Some(DashType::Forward),
                CommandMotion::Backdash => Some(DashType::Back),
                _ => None,
            }
        } else {
            None
        }
    }
}

pub enum DashType {
    Forward,
    Back,
}

pub const A: ButtonMask = ButtonMask(0b0000_0001);
pub const B: ButtonMask = ButtonMask(0b0000_0010);
pub const C: ButtonMask = ButtonMask(0b0000_0100);
pub const D: ButtonMask = ButtonMask(0b0000_1000);
pub const E: ButtonMask = ButtonMask(0b0001_0000);
pub const F: ButtonMask = ButtonMask(0b0010_0000);
pub const G: ButtonMask = ButtonMask(0b0100_0000);
pub const H: ButtonMask = ButtonMask(0b1000_0000);

#[derive(Debug, Default, Clone, Copy, Reflect, FromReflect, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct ButtonMask(pub u8);

impl ButtonMask {
    pub fn new(value: u8) -> Self {
        ButtonMask(value)
    }

    pub fn with_buttons(buttons: &str) -> Self {
        let mut binary_representation = 0_u8;
        for button in buttons.chars().into_iter() {
            let bit_to_set = match button {
                'a' => 0b0000_0001,
                'b' => 0b0000_0010,
                'c' => 0b0000_0100,
                'd' => 0b0000_1000,
                'e' => 0b0001_0000,
                'f' => 0b0010_0000,
                'g' => 0b0100_0000,
                'h' => 0b1000_0000,
                _ => 0,
            };
            binary_representation |= bit_to_set;
        }

        ButtonMask(binary_representation)
    }

    pub fn raw_value(&self) -> u8 {
        self.0
    }

    pub fn any(&self) -> bool {
        self.0 != 0
    }

    pub fn contains(&self, button: char) -> bool {
        let shift: u8 = match button {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return false,
        };

        self.is_bit_set(shift)
    }

    fn is_bit_set(&self, position: u8) -> bool {
        (self.0 & (1 << position)) != 0
    }
}

impl fmt::Display for ButtonMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 & A.0 == A.0 {
            write!(f, "a")?
        }
        if self.0 & B.0 == B.0 {
            write!(f, "b")?
        }
        if self.0 & C.0 == C.0 {
            write!(f, "c")?
        }
        if self.0 & D.0 == D.0 {
            write!(f, "d")?
        }
        if self.0 & E.0 == E.0 {
            write!(f, "e")?
        }
        if self.0 & F.0 == F.0 {
            write!(f, "f")?
        }
        if self.0 & G.0 == G.0 {
            write!(f, "g")?
        }
        if self.0 & H.0 == H.0 {
            write!(f, "h")?
        }
        Ok(())
    }
}

impl std::ops::BitAnd for ButtonMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

#[derive(Clone)]
pub struct ButtonStream {
    held_buttons: Vec<ButtonMask>,
    pressed_buttons: Vec<ButtonMask>,
    released_buttons: Vec<ButtonMask>,
}

impl ButtonStream {
    pub fn replace(&mut self, held: ButtonMask, pressed: ButtonMask, released: ButtonMask) {
        self.held_buttons.remove(0);
        self.held_buttons.push(held);
        self.pressed_buttons.remove(0);
        self.pressed_buttons.push(pressed);
        self.released_buttons.remove(0);
        self.released_buttons.push(released);
    }

    pub fn with_buttons(held: ButtonMask, pressed: ButtonMask, released: ButtonMask) -> Self {
        ButtonStream {
            held_buttons: vec![held; 15],
            pressed_buttons: vec![pressed; 15],
            released_buttons: vec![released; 15],
        }
    }

    pub fn held_in_range(&self, start: usize, end: usize) -> ButtonMask {
        let mut held: u8 = 0;
        for button in self.held_buttons[start..end].iter() {
            held |= button.raw_value();
        }
        ButtonMask::new(held)
    }

    pub fn pressed_in_range(&self, start: usize, end: usize) -> ButtonMask {
        let mut pressed: u8 = 0;
        for button in self.pressed_buttons[start..end].iter() {
            pressed |= button.raw_value();
        }
        ButtonMask::new(pressed)
    }

    pub fn released_in_range(&self, start: usize, end: usize) -> ButtonMask {
        let mut released: u8 = 0;
        for button in self.released_buttons[start..end].iter() {
            released |= button.raw_value();
        }
        ButtonMask::new(released)
    }

    pub fn buffered(&self) -> ButtonMask {
        let pressed_len = self.pressed_buttons.len();
        let pressed_end = if pressed_len > 0 { pressed_len - 1 } else { 0 };
        self.buffered_at_index(pressed_end)
    }

    pub fn buffered_at_index(&self, end: usize) -> ButtonMask {
        let start = if end < 5 { 0 } else { end - 5 };
        let pressed_mask = self.pressed_in_range(start, end);
        let held_mask = self.held_in_range(start, end);
        ButtonMask::new(pressed_mask.raw_value() & held_mask.raw_value())
    }
}

impl Default for ButtonStream {
    fn default() -> Self {
        let buttons = vec![ButtonMask::new(0); 15];

        ButtonStream {
            held_buttons: buttons.clone(),
            pressed_buttons: buttons.clone(),
            released_buttons: buttons.clone(),
        }
    }
}

pub const LEFT: MotionMask = MotionMask(0b0000_0001);
pub const RIGHT: MotionMask = MotionMask(0b0000_0010);
pub const DOWN: MotionMask = MotionMask(0b0000_0100);
pub const UP: MotionMask = MotionMask(0b0000_1000);
pub const DOWN_LEFT: MotionMask = MotionMask(0b0000_0101);
pub const DOWN_RIGHT: MotionMask = MotionMask(0b0000_0110);
pub const UP_LEFT: MotionMask = MotionMask(0b0000_1001);
pub const UP_RIGHT: MotionMask = MotionMask(0b0000_1010);
pub const NEUTRAL: MotionMask = MotionMask(0b0000_0000);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Reflect, FromReflect)]
#[repr(transparent)]
pub struct MotionMask(u8);

impl MotionMask {
    pub fn new(mut value: u8) -> Self {
        const LEFT_POS: u8 = 0;
        const RIGHT_POS: u8 = 1;
        const DOWN_POS: u8 = 2;
        const UP_POS: u8 = 3;

        const LEFT: u8 = 0b0000_0001;
        const RIGHT: u8 = 0b0000_0010;
        const DOWN: u8 = 0b0000_0100;

        fn is_bit_set(value: u8, position: u8) -> bool {
            (value & (1 << position)) != 0
        }

        fn unset_horizontal_bits(mut value: u8) -> u8 {
            value &= LEFT;
            value &= RIGHT;
            value
        }

        fn unset_the_down_bit(mut value: u8) -> u8 {
            value &= DOWN;
            value
        }

        if is_bit_set(value, LEFT_POS) && is_bit_set(value, RIGHT_POS) {
            value = unset_horizontal_bits(value);
        }

        if is_bit_set(value, DOWN_POS) && is_bit_set(value, UP_POS) {
            value = unset_the_down_bit(value);
        }

        MotionMask(value)
    }

    pub fn is_down(self) -> bool {
        matches!(self, DOWN | DOWN_LEFT | DOWN_RIGHT)
    }

    pub fn is_right(self) -> bool {
        matches!(self, RIGHT | DOWN_RIGHT | UP_RIGHT)
    }

    pub fn is_left(self) -> bool {
        matches!(self, LEFT | DOWN_LEFT | UP_LEFT)
    }

    pub fn is_up(self) -> bool {
        matches!(self, UP | UP_LEFT | UP_RIGHT)
    }

    pub fn with_direction(motion: &str) -> Self {
        let mut binary_representation = 0_u8;
        for dir in motion.chars().into_iter() {
            let bit_to_set = match dir {
                'l' => 0b0000_0001,
                'r' => 0b0000_0010,
                'd' => 0b0000_0100,
                'u' => 0b0000_1000,
                _ => 0,
            };
            binary_representation |= bit_to_set;
        }
        MotionMask::new(binary_representation)
    }

    pub fn raw_value(&self) -> u8 {
        self.0
    }

    pub fn to_unicode(&self) -> char {
        match *self {
            LEFT => char::from_u32(0x2190).unwrap(),
            RIGHT => char::from_u32(0x2192).unwrap(),
            DOWN => char::from_u32(0x2193).unwrap(),
            UP => char::from_u32(0x2191).unwrap(),
            DOWN_LEFT => char::from_u32(0x2199).unwrap(),
            DOWN_RIGHT => char::from_u32(0x2198).unwrap(),
            UP_LEFT => char::from_u32(0x2196).unwrap(),
            UP_RIGHT => char::from_u32(0x2197).unwrap(),
            NEUTRAL => char::from_u32(0x2605).unwrap(),
            _ => ' ',
        }
    }

    pub fn to_numpad(&self, facing_right: bool) -> u8 {
        if facing_right {
            match *self {
                LEFT => 4,
                RIGHT => 6,
                DOWN => 2,
                UP => 8,
                DOWN_LEFT => 1,
                DOWN_RIGHT => 3,
                UP_LEFT => 7,
                UP_RIGHT => 9,
                NEUTRAL => 5,
                _ => 0,
            }
        } else {
            match *self {
                LEFT => 6,
                RIGHT => 4,
                DOWN => 2,
                UP => 8,
                DOWN_LEFT => 3,
                DOWN_RIGHT => 1,
                UP_LEFT => 9,
                UP_RIGHT => 7,
                NEUTRAL => 5,
                _ => 0,
            }
        }
    }
}

impl fmt::Display for MotionMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_unicode())
    }
}

pub struct MotionStream {
    motions: Vec<MotionMask>,
}

impl MotionStream {
    pub fn replace(&mut self, motion: MotionMask) {
        self.motions.remove(0);
        self.motions.push(motion);
    }

    pub fn to_numpad(&self, facing_right: bool) -> String {
        self.motions
            .iter()
            .map(|i| i.to_numpad(facing_right).to_string())
            .collect()
    }
    pub fn last_motion(&self) -> MotionMask {
        *self.motions.last().unwrap()
    }
}

impl Default for MotionStream {
    fn default() -> Self {
        let motions = vec![NEUTRAL; 15];
        MotionStream { motions }
    }
}

#[derive(Debug, Default, Clone, Copy, Reflect, FromReflect)]
pub struct InputMask {
    motion: MotionMask,
    held_buttons: ButtonMask,
    pressed_buttons: ButtonMask,
    released_buttons: ButtonMask,
}

impl InputMask {
    pub fn new(
        motion: MotionMask,
        held_buttons: ButtonMask,
        pressed_buttons: ButtonMask,
        released_buttons: ButtonMask,
    ) -> Self {
        InputMask {
            motion,
            held_buttons,
            pressed_buttons,
            released_buttons,
        }
    }
    pub fn motion(&self) -> MotionMask {
        self.motion
    }

    pub fn held_buttons(&self) -> ButtonMask {
        self.held_buttons
    }

    pub fn pressed_buttons(&self) -> ButtonMask {
        self.pressed_buttons
    }

    pub fn released_buttons(&self) -> ButtonMask {
        self.released_buttons
    }
}

impl fmt::Display for InputMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let motion_string = self.motion().to_string();
        let held_button_string = self.held_buttons().to_string();
        let pressed_button_string = self.pressed_buttons().to_string();
        let released_button_string = self.released_buttons().to_string();
        write!(
            f,
            "Motion: {}\nHeld Buttons: {}\nPressed Buttons: {}\nReleased Buttons: {}",
            motion_string, held_button_string, pressed_button_string, released_button_string
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn button_stream() {
        let stream = ButtonStream {
            held_buttons: vec![
                ButtonMask::with_buttons("d"),
                ButtonMask::with_buttons(""),
                ButtonMask::with_buttons("ac"),
                ButtonMask::with_buttons("g"),
                ButtonMask::with_buttons("h"),
                ButtonMask::with_buttons("e"),
            ],
            pressed_buttons: Vec::new(),
            released_buttons: Vec::new(),
        };

        assert_eq!(stream.held_in_range(0, 5).to_string(), "acdgh".to_string());
    }

    #[test]
    fn input_mask_test() {
        let mask = InputMask {
            held_buttons: ButtonMask::with_buttons("abc"),
            pressed_buttons: ButtonMask::with_buttons("abc"),
            released_buttons: ButtonMask::with_buttons("abc"),
            motion: MotionMask::with_direction("dr"),
        };
        assert_eq!(
            mask.to_string(),
            "Motion: ↘\nHeld Buttons: abc\nPressed Buttons: abc\nReleased Buttons: abc".to_string()
        )
    }

    #[test]
    fn detect_single_button() {
        let single_button_mask = ButtonMask(0b0000_1000);
        assert!(single_button_mask.contains('d'));
    }

    #[test]
    fn detect_multiple_buttons() {
        let multi_button_mask = ButtonMask(0b1010_0110);
        assert!(
            multi_button_mask.contains('h')
                && multi_button_mask.contains('f')
                && multi_button_mask.contains('b')
                && multi_button_mask.contains('c')
        )
    }

    #[test]
    fn motion_mask_to_numpad() {
        //
        let down_mask = MotionMask::new(0b0000_0100);
        let up_mask = MotionMask::new(0b0000_1000);
        let left_mask = MotionMask::new(0b0000_0001);
        let right_mask = MotionMask::new(0b0000_0010);
        let down_right_mask = MotionMask::new(0b0000_0110);
        let down_left_mask = MotionMask::new(0b0000_0101);
        let up_left_mask = MotionMask::new(0b0000_1001);
        let up_right_mask = MotionMask::new(0b0000_1010);
        let neutral_mask = MotionMask::new(0b0000_0000);
        // Universal Motions
        assert_eq!(neutral_mask.to_numpad(false), 5);
        assert_eq!(down_mask.to_numpad(false), 2);
        assert_eq!(up_mask.to_numpad(false), 8);

        // P1 Motions
        assert_eq!(left_mask.to_numpad(true), 4);
        assert_eq!(right_mask.to_numpad(true), 6);
        assert_eq!(down_left_mask.to_numpad(true), 1);
        assert_eq!(down_right_mask.to_numpad(true), 3);
        assert_eq!(up_left_mask.to_numpad(true), 7);
        assert_eq!(up_right_mask.to_numpad(true), 9);

        // P2 Motions
        assert_eq!(left_mask.to_numpad(false), 6);
        assert_eq!(right_mask.to_numpad(false), 4);
        assert_eq!(down_left_mask.to_numpad(false), 3);
        assert_eq!(down_right_mask.to_numpad(false), 1);
        assert_eq!(up_left_mask.to_numpad(false), 9);
        assert_eq!(up_right_mask.to_numpad(false), 7);
    }

    #[test]
    fn motion_mask_to_unicode() {
        let down_mask = MotionMask::new(0b0000_0100);
        let up_mask = MotionMask::new(0b0000_1000);
        let left_mask = MotionMask::new(0b0000_0001);
        let right_mask = MotionMask::new(0b0000_0010);
        let down_right_mask = MotionMask::new(0b0000_0110);
        let down_left_mask = MotionMask::new(0b0000_0101);
        let up_left_mask = MotionMask::new(0b0000_1001);
        let up_right_mask = MotionMask::new(0b0000_1010);
        let neutral_mask = MotionMask::new(0b0000_0000);

        assert_eq!('↓', down_mask.to_unicode());
        assert_eq!('↑', up_mask.to_unicode());
        assert_eq!('←', left_mask.to_unicode());
        assert_eq!('→', right_mask.to_unicode());
        assert_eq!('↙', down_left_mask.to_unicode());
        assert_eq!('↘', down_right_mask.to_unicode());
        assert_eq!('↖', up_left_mask.to_unicode());
        assert_eq!('↗', up_right_mask.to_unicode());
        assert_eq!('★', neutral_mask.to_unicode());
    }
}
