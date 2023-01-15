use crate::*;

#[derive(Debug)]
/// Event generated by each input device each frame to read player inputs
pub struct InputEvent {
    pub player_id: PlayerId,
    /// Direction of the input, expressed in numpad notation
    pub motion: u8,
    /// Reperesents what buttons were pressed, held, and released this frame
    pub buttons: Buttons,
    /// Any command motion found in the input event
    pub special_motion: Option<CommandType>,
    /// Duration of the current command motion
    pub special_motion_duration: u8,
}

impl InputEvent {
    pub fn new(motion: u8, player_id: PlayerId, buttons: Buttons) -> Self {
        InputEvent {
            motion,
            player_id,
            buttons,
            special_motion_duration: 0,
            special_motion: None,
        }
    }
}