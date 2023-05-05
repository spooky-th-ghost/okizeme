use crate::*;

pub struct InputBuffer {
    pub player_id: PlayerId,
    pub inputs: Vec<InputMask>,
}
impl InputBuffer {
    pub fn with_player(player_id: PlayerId) -> Self {
        InputBuffer {
            player_id,
            inputs: Vec::new(),
        }
    }

    pub fn get_player_id(&self) -> &PlayerId {
        &self.player_id
    }

    pub fn tick(&mut self) {
        if self.inputs.len() > 15 {
            self.inputs.remove(0);
        }
    }

    pub fn update(&mut self, input_event: &InputEvent) {
        self.tick();
        self.inputs.push(input_event.input_mask.clone());
    }

    pub fn motions_to_numpad(&self, facing_right: bool) -> String {
        self.inputs
            .iter()
            .map(|i| i.motion.to_numpad(facing_right) as char)
            .collect()
    }
}

pub struct ButtonStream {
    pub held_buttons: Vec<ButtonMask>,
    pub pressed_buttons: Vec<ButtonMask>,
    pub released_buttons: Vec<ButtonMask>,
}

impl ButtonStream {
    pub fn held_in_range(&self, start: usize, end: usize) -> ButtonMask {
        let mut held: u8 = 0;
        for button in self.held_buttons[start..end].iter() {
            held |= button.raw_value();
        }
        ButtonMask::new(held)
    }

    pub fn pressed_in_range(&self, start: usize, end: usize) -> ButtonMask {
        let mut pressed: u8 = 0;
        for button in self.held_buttons[start..end].iter() {
            pressed |= button.raw_value();
        }
        ButtonMask::new(pressed)
    }
}

#[test]
fn test_button_stream() {
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

    assert_eq!(stream.held_in_range(0, 5).to_string(), "a".to_string());
}
