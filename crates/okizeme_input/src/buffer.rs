use std::fmt::Write;
use okizeme_types::PlayerId;
use okizeme_utils::countdown;

use crate::{
  CommandType,
  ButtonPress,
  MOTIONS,
  InputEvent
};

/// Input buffer used to hold button presses, directional input, and special motions
#[derive(Debug, Clone)]
pub struct Buffer {
  motions: Vec<u8>,
  command_priority: u8,
  command_duration: u8,
  command_type: Option<CommandType>,
  current_motion: u8,
  current_press: ButtonPress,
  previous_motion: u8,
  command_lockout: u8,
  buffer_size: usize,
}

impl Buffer {
  pub fn new() -> Self {
    Buffer {
      motions: Vec::new(),
      command_priority: 0,
      command_duration: 0,
      command_type: None,
      current_motion: 5,
      current_press: ButtonPress::new(0),
      previous_motion: 5,
      command_lockout: 0,
      buffer_size: 20,
    }
  }

  pub fn update(&mut self, event: &InputEvent) {
    self.tick();
    self.motions.push(event.motion);
    self.previous_motion = self.current_motion;
    self.current_motion = event.motion;
    self.current_press = event.button_press; 
    self.extract_special_motions();
  }

  pub fn current_input(&self) -> String {
    return format!("{:?}{}", self.current_motion, &self.current_press.to_string()[..]);
  }

  fn tick(&mut self) {
    if self.motions.len() > self.buffer_size {
      self.motions.remove(0);
    }

    if self.command_duration == 0 {
      self.command_type = None;
    }

    self.command_duration = countdown(self.command_duration);
    self.command_lockout = countdown(self.command_lockout);
  }

  fn motion_to_string(&mut self) -> String {
    let mut motions_string = String::new();
    for motion in self.motions.iter() {
      write!(motions_string,"{:?}",motion).unwrap();
    }
    motions_string
  }

  pub fn consume_motion(&mut self) {
    self.command_type = None;
    self.command_lockout = 3;
    self.command_duration = 0;
  }
  
  fn extract_special_motions(&mut self) {
    if self.command_lockout == 0 {
      let motion_string = self.motion_to_string();
      let mut priority: u8 = self.command_priority;
      let mut current_command: Option<CommandType> = None;

      for command_motion in MOTIONS.iter() {
        if  command_motion.check(&motion_string[..], priority) {
          priority = command_motion.priority;
          current_command = Some(command_motion.command);
        }
      }

      if let Some(c) = current_command {
        self.command_type = Some(c);
        self.command_duration = 5;
      }
    }
  }

    pub fn get_current_motion(&self) -> u8 {
        self.current_motion
    }

    pub fn get_command_type(&self) -> Option<CommandType> {
        self.command_type
    }

    pub fn get_current_press(&self) -> ButtonPress {
        self.current_press
    }
}
