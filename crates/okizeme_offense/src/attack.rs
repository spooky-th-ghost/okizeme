use bevy_inspector_egui::Inspectable;
use crate::Hitbox;

#[derive(Debug, Clone, Inspectable)]
pub struct Attack {
  pub name: String,
  pub duration: u8,
  pub attack_events: Vec<AttackEvent>
}


pub struct Attacking {
    counter_hit: bool,
    cancellable: bool,
    duration: u8
}

impl Default for Attack {
  fn default() -> Self {
      Attack { 
          name: "none".to_string(),
          duration: 0,
          attack_events: Vec::new() 
      }
  }
}

#[derive(Debug, Default, Clone, Inspectable)]
pub struct AttackEvent {
  pub frame: u8,
  pub hitbox: Hitbox,
}
