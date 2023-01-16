use crate::*;
use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, *};

#[derive(Component)]
pub struct InputListener {
    pub player_id: PlayerId,
}

#[derive(Bundle)]
pub struct InputListenerBundle {
    input_listener: InputListener,
    #[bundle]
    input_manager: InputManagerBundle<OkiAction>,
}

impl InputListenerBundle {
    pub fn input_map(player_id: PlayerId) -> input_map::InputMap<OkiAction> {
        use OkiAction::*;
        let mut input_map = match player_id {
            PlayerId::P1 => input_map::InputMap::new([
                (KeyCode::Q, Left),
                (KeyCode::W, Down),
                (KeyCode::E, Right),
                (KeyCode::Space, Up),
                (KeyCode::Y, A),
                (KeyCode::U, B),
                (KeyCode::I, C),
                (KeyCode::O, G),
                (KeyCode::H, D),
                (KeyCode::J, E),
                (KeyCode::K, F),
                (KeyCode::L, H),
            ])
            .build(),
            PlayerId::P2 => input_map::InputMap::new([
                (KeyCode::I, Left),
                (KeyCode::O, Down),
                (KeyCode::P, Right),
                (KeyCode::N, Up),
                (KeyCode::Z, A),
                (KeyCode::X, B),
                (KeyCode::C, C),
                (KeyCode::V, G),
                (KeyCode::B, D),
                (KeyCode::N, E),
                (KeyCode::M, F),
                (KeyCode::A, H),
            ])
            .build(),
        };
        input_map
    }
}

pub fn write_inputs_to_buffer(
    mut input_writer: EventWriter<InputEvent>,
    query: Query<(
        &InputListener,
        &leafwing_input_manager::action_state::ActionState<OkiAction>,
    )>,
) {
    for (listener, action) in &query {
        use OkiAction::*;

        let a_pressed = action.just_pressed(A);
        let a_held = action.pressed(A);
        let a_released = action.just_released(A);

        let b_pressed = action.just_pressed(B);
        let b_held = action.pressed(B);
        let b_released = action.just_released(B);

        let c_pressed = action.just_pressed(C);
        let c_held = action.pressed(C);
        let c_released = action.just_released(C);

        let d_pressed = action.just_pressed(D);
        let d_held = action.pressed(D);
        let d_released = action.just_released(D);

        let e_pressed = action.just_pressed(E);
        let e_held = action.pressed(E);
        let e_released = action.just_released(E);

        let f_pressed = action.just_pressed(F);
        let f_held = action.pressed(F);
        let f_released = action.just_released(F);

        let g_pressed = action.just_pressed(G);
        let g_held = action.pressed(G);
        let g_released = action.just_released(G);

        let h_pressed = action.just_pressed(H);
        let h_held = action.pressed(H);
        let h_released = action.just_released(H);

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

        // Motion
        let up = action.pressed(Up);
        let down = action.pressed(Down);
        let left = action.pressed(Left);
        let right = action.pressed(Right);

        let mut h_axis: f32 = 0.0;
        let mut v_axis: f32 = 0.0;

        if left {
            h_axis -= 1.0;
        }

        if right {
            h_axis += 1.0;
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
        input_writer.send(InputEvent::new(motion, listener.player_id, buttons));
    }
}
