use regex::Regex;

/// Notation, Priority, and Regex for special motions
#[derive(Debug)]
pub struct CommandMotion {
  pub priority: u8,
  regular_expression: Regex,
  pub command: CommandType
}

impl CommandMotion {
  pub fn new(priority: u8, regular_expression: Regex, command: CommandType) -> Self {
    CommandMotion { 
      priority, 
      regular_expression,
      command
    }
  }

  pub fn check(&self, buffer_string: &str, buffer_priority: u8) -> bool {
    self.regular_expression.is_match(buffer_string) && self.priority > buffer_priority
  }


}

#[derive(Debug, Clone, Copy)]
pub enum CommandType {
    Qcf,
    Qcb,
    Dp,
    BackDp,
    Hcb,
    Hcf,
    Dash,
    BackDash,
    TwoTwo
}
