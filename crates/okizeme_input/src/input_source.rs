pub use crate::{
    CommandType,
    ButtonPress
};

/// Trait for any struct that reports player input
pub trait InputSource {
    fn get_current_motion(&self) -> u8;
    fn get_command_type(&self) -> Option<CommandType>;
    fn get_current_press(&self) -> ButtonPress;
    fn get_current_input(&self) -> String {
        format!("{:?}{}", self.get_current_motion(), &self.get_current_press().to_string()[..])
    }
}
