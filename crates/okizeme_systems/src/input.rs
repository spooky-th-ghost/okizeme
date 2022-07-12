use bevy::prelude::*;
use okizeme_input::{
    InputMap,
    InputEvent,
    InputActionsPressed,
    ButtonPress,
    InputMethod
};
use okizeme_resources::{
    PlayerInputSources, 
    PlayerDevices,
    PlayerPositions
};

pub fn write_inputs(
  player_devices: Res<PlayerDevices>,
  player_positions: Res<PlayerPositions>,
  keyboard_input: Res<Input<KeyCode>>, 
  button_input: Res<Input<GamepadButton>>,
  mut input_writer: EventWriter<InputEvent>
) {
  for mapper in player_devices.get().iter() {
    let mut h_axis: f32 = 0.0;
    let mut v_axis: f32 = 0.0;
    let InputMap { player_id, ..} = mapper;
    
    let InputActionsPressed {
      right, 
      left, 
      up, 
      down,
      a, 
      b, 
      c, 
      d,
      e,
      f,
      macro_1,
      macro_2} = mapper.get_pressed_buttons(&keyboard_input, &button_input);


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


    let mut pressed_byte: u8 = 0b0000_0000;
    if a {pressed_byte |= 0b0000_0001}
    if b {pressed_byte |= 0b0000_0010}
    if c {pressed_byte |= 0b0000_0100}
    if d {pressed_byte |= 0b0000_1000}
    if e {pressed_byte |= 0b0001_0000}
    if f {pressed_byte |= 0b0010_0000}
    if macro_1 {pressed_byte |= 0b0100_0000}
    if macro_2 {pressed_byte |= 0b1000_0000}
    let button_press = ButtonPress::new(pressed_byte);
    input_writer.send(
        InputEvent::new(
            motion,
            *player_id,
            button_press
        )
    );
  }
}

pub fn read_inputs(
  mut input_reader: EventReader<InputEvent>, 
  mut player_buffers: ResMut<PlayerInputSources>,
) {
    for event in input_reader.iter() {
        let buffer = player_buffers.get_source_mut(&event.player_id);
        buffer.update(event);
    };
}
