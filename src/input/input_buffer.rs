use crate::{ButtonStream, InputEvent, MotionStream, PlayerId};
use bevy::prelude::*;

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

#[derive(Resource)]
pub struct PlayerInputSources(Vec<InputBuffer>);

impl Default for PlayerInputSources {
    fn default() -> Self {
        PlayerInputSources(vec![
            InputBuffer::new(PlayerId::P1),
            InputBuffer::new(PlayerId::P2),
        ])
    }
}

impl PlayerInputSources {
    pub fn get_source_mut(&mut self, player_id: &PlayerId) -> &mut InputBuffer {
        self.0
            .iter_mut()
            .find(|x| x.get_player_id() == player_id)
            .unwrap()
    }
    pub fn get_source(&self, player_id: &PlayerId) -> &InputBuffer {
        &self
            .0
            .iter()
            .find(|&x| x.get_player_id() == player_id)
            .unwrap()
    }
}

pub fn read_inputs(
    mut input_reader: EventReader<InputEvent>,
    mut player_buffers: ResMut<PlayerInputSources>,
) {
    for event in input_reader.iter() {
        let buffer = player_buffers.get_source_mut(&event.player_id);
        buffer.update(event);
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
