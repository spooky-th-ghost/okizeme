use crate::*;
use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, *};

#[derive(Component)]
pub struct InputListener {
    pub player_id: PlayerId,
}

impl InputListener {
    pub fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}

#[derive(Bundle)]
pub struct InputListenerBundle {
    input_listener: InputListener,
    #[bundle]
    input_manager: InputManagerBundle<OkiAction>,
}

impl InputListenerBundle {
    pub fn input_map(player_id: PlayerId) -> InputListenerBundle {
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
        InputListenerBundle {
            input_listener: InputListener::new(player_id),
            input_manager: InputManagerBundle {
                input_map,
                ..Default::default()
            },
        }
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

        let a = action.pressed(A);

        let b = action.pressed(B);

        let c = action.pressed(C);

        let d = action.pressed(D);

        let e = action.pressed(E);

        let f = action.pressed(F);

        let g = action.pressed(G);

        let h = action.pressed(H);

        let mut button_byte: u8 = 0b0000_0000;
        if a {
            button_byte |= 0b0000_0001
        }
        if b {
            button_byte |= 0b0000_0010
        }
        if c {
            button_byte |= 0b0000_0100
        }
        if d {
            button_byte |= 0b0000_1000
        }
        if e {
            button_byte |= 0b0001_0000
        }
        if f {
            button_byte |= 0b0010_0000
        }
        if g {
            button_byte |= 0b0100_0000
        }
        if h {
            button_byte |= 0b1000_0000
        }

        let button_mask = ButtonMask::new(button_byte);

        // Motion
        let up = action.pressed(Up);
        let down = action.pressed(Down);
        let left = action.pressed(Left);
        let right = action.pressed(Right);

        let mut motion_byte: u8 = 0b0000_0000;
        if left {
            motion_byte |= 0b0000_0001
        }
        if right {
            motion_byte |= 0b0000_0010
        }
        if down {
            motion_byte |= 0b0000_0100
        }
        if up {
            motion_byte |= 0b0000_1000
        }

        let motion_mask = MotionMask::new(motion_byte);

        let input_mask = InputMask::from_masks(button_mask, motion_mask);

        input_writer.send(InputEvent::new(listener.player_id, input_mask));
    }
}
