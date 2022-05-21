use bevy_inspector_egui::Inspectable;
use crate::{
  Hitbox
};

#[derive(Debug, Clone, Inspectable)]
pub struct Attack {
  pub name: String,
  pub duration: u8,
  pub attack_events: Vec<AttackEvent>
}

impl Default for Attack {
  fn default() -> Self {
      Attack { name: "none".to_string(), duration: 0, attack_events: Vec::new() }
  }
}


#[derive(Debug, Clone, Inspectable)]
pub struct AttackEvent {
  pub frame: u8,
  pub hitbox: Hitbox,
}

impl Default for AttackEvent {
  fn default() -> Self {
      AttackEvent {
        frame: 0,
        hitbox: Hitbox::default()
      }
  }
}


