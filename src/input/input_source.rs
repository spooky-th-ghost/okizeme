use crate::*;

pub trait InputMethod {
    fn get_current_motion(&self) -> u8;
    fn get_command_type(&self) -> Option<CommandType>;
    fn get_current_hold(&self) -> ButtonMask;
    fn get_current_press(&self) -> ButtonMask;
    fn get_current_release(&self) -> ButtonMask;
    fn get_player_id(&self) -> &PlayerId;
    fn update(&mut self, event: &InputEvent);
}

pub enum InputSource {
    Buffer(Buffer),
}

impl InputSource {
    pub fn new_buffer(player_id: PlayerId) -> Self {
        InputSource::Buffer(Buffer::new(player_id))
    }
}

impl InputMethod for InputSource {
    fn get_current_motion(&self) -> u8 {
        match self {
            InputSource::Buffer(buffer) => buffer.get_current_motion(),
        }
    }

    fn get_command_type(&self) -> Option<CommandType> {
        match self {
            InputSource::Buffer(buffer) => buffer.get_command_type(),
        }
    }

    fn get_current_release(&self) -> ButtonMask {
        match self {
            InputSource::Buffer(buffer) => buffer.get_current_release(),
        }
    }

    fn get_current_hold(&self) -> ButtonMask {
        match self {
            InputSource::Buffer(buffer) => buffer.get_current_hold(),
        }
    }

    fn get_current_press(&self) -> ButtonMask {
        match self {
            InputSource::Buffer(buffer) => buffer.get_current_press(),
        }
    }

    fn get_player_id(&self) -> &PlayerId {
        match self {
            InputSource::Buffer(buffer) => buffer.get_player_id(),
        }
    }

    fn update(&mut self, event: &InputEvent) {
        match self {
            InputSource::Buffer(buffer) => buffer.update(event),
        }
    }
}
