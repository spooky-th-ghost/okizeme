use crate::{ButtonStream, InputEvent, MotionStream, PlayerId};

pub struct InputBuffer {
    pub player_id: PlayerId,
    pub motions: MotionStream,
    pub buttons: ButtonStream,
}
impl InputBuffer {
    pub fn new(player_id: PlayerId) -> Self {
        InputBuffer {
            player_id,
            motions: MotionStream::default(),
            buttons: ButtonStream::default(),
        }
    }

    pub fn get_player_id(&self) -> &PlayerId {
        &self.player_id
    }

    pub fn update(&mut self, input_event: &InputEvent) {
        let InputEvent {
            player_id: _,
            input_mask,
        } = input_event;
        self.motions.replace(input_mask.motion());
        self.buttons.replace(
            input_mask.held_buttons(),
            input_mask.pressed_buttons(),
            input_mask.released_buttons(),
        );
    }

    pub fn motions_to_numpad(&self, facing_right: bool) -> String {
        self.motions.to_numpad(facing_right)
    }
}

#[test]
fn new_buffer() {
    let buffer = InputBuffer::new(PlayerId::P1);
    assert_eq!(
        buffer.motions_to_numpad(true),
        "555555555555555".to_string()
    );
}
