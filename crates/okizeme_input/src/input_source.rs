use crate::InputEvent;
pub use crate::{
    CommandType,
    ButtonPress,
    Buffer
};

pub use okizeme_types::PlayerId;

pub trait InputMethod {
    fn get_current_motion(&self) -> u8;
    fn get_command_type(&self) -> Option<CommandType>;
    fn get_current_press(&self) -> ButtonPress;
    fn get_player_id(&self) -> &PlayerId;
    fn update(&mut self, event: &InputEvent);
}

pub enum InputSource {
    Buffer(Buffer)
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

    fn get_current_press(&self) -> ButtonPress {
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
// pub struct InputSource {
//     pub player_id: PlayerId,
//     source: InputSourceType
// }
// /// Enum used to handle all possible input sources
// #[derive(Clone)]
// pub enum InputSourceType {
//     LocalBuffer(Buffer),
//     NetworkBuffer,
//     ReplayBuffer,
//     TrainingBuffer,
//     AiBuffer
// }

// impl InputSource {
//     pub fn buffer(player_id: PlayerId) -> Self {
//         InputSource {
//             player_id,
//             source: InputSourceType::LocalBuffer(Buffer::new())
//         }
//     }

//     pub fn update(&mut self, event: &InputEvent) {
//         // let a = self.source.to_owned();
//         match self.source.to_owned() {
//             InputSourceType::LocalBuffer(mut b) => {
//                 b.update(&event);
//             },
//             _ => ()
//         }
//     }
//     pub fn get_current_motion(&self) -> u8 {
//         match &self.source {
//             InputSourceType::LocalBuffer(b) => b.get_current_motion(),
//             _ => 5
//         }
//     }
//     pub fn get_command_type(&self) -> Option<CommandType> {
//         match &self.source {
//             InputSourceType::LocalBuffer(b) => b.get_command_type(),
//             _ => None
//         }
//     }
//     pub fn get_current_press(&self) -> ButtonPress {
//         match &self.source {
//             InputSourceType::LocalBuffer(b) => b.get_current_press(),
//             _ => ButtonPress::new(0) 
//         }
//     }
//     pub fn get_current_input(&self) -> String {
//         format!("{:?}{}", self.get_current_motion(), &self.get_current_press().to_string()[..])
//     }
// }
