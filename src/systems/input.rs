use crate::{
    InputEvent, InputMap, InputMethod, PlayerDevices, PlayerInputSources, PlayerPositions,
    RawInputFrame,
};
use bevy::prelude::*;

pub fn write_inputs(
    player_devices: Res<PlayerDevices>,
    player_positions: Res<PlayerPositions>,
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    mut input_writer: EventWriter<InputEvent>,
) {
    for mapper in player_devices.get().iter() {
        let mut h_axis: f32 = 0.0;
        let mut v_axis: f32 = 0.0;
        let InputMap { player_id, .. } = mapper;

        let RawInputFrame {
            buttons,
            right,
            left,
            up,
            down,
        } = mapper.get_raw_input_frame(&keyboard_input, &button_input);

        if left {
            h_axis -= 1.0 * player_positions.get_facing_vector(player_id);
        }

        if right {
            h_axis += 1.0 * player_positions.get_facing_vector(player_id);
        }

        if up {
            v_axis = 1.0;
        }

        if down {
            if v_axis == 0.0 {
                v_axis = -1.0;
            }
        }

        let mut motion: u8 = 5;

        if h_axis == 0.0 {
            if v_axis == 1.0 {
                motion = 8;
            }

            if v_axis == -1.0 {
                motion = 2;
            }
        }

        if h_axis == -1.0 {
            if v_axis == 1.0 {
                motion = 7;
            }

            if v_axis == 0.0 {
                motion = 4;
            }

            if v_axis == -1.0 {
                motion = 1;
            }
        }

        if h_axis == 1.0 {
            if v_axis == 1.0 {
                motion = 9;
            }

            if v_axis == 0.0 {
                motion = 6;
            }

            if v_axis == -1.0 {
                motion = 3;
            }
        }

        input_writer.send(InputEvent::new(motion, *player_id, buttons));
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
