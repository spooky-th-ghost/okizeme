use crate::{InputEvent, PlayerInputSources};
use bevy::prelude::*;

pub fn read_inputs(
    mut input_reader: EventReader<InputEvent>,
    mut player_buffers: ResMut<PlayerInputSources>,
) {
    for event in input_reader.iter() {
        let buffer = player_buffers.get_source_mut(&event.player_id);
        buffer.update(event);
    }
}
