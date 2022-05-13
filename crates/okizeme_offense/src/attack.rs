use bevy_inspector_egui::Inspectable;
use crate::{
  Hitbox
};

//#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Inspectable)]
pub struct Attack {
  pub attack_events: Vec<AttackEvent>
}

impl Default for Attack {
  fn default() -> Self {
      Attack { attack_events: Vec::new() }
  }
}

//#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
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


