use bevy::{
    prelude::*,
    core::FixedTimestep
};

pub use okizeme_core::*;

pub mod prelude {
    pub use crate::OkizemePlugin;
}

pub struct OkizemePlugin;

impl Plugin for OkizemePlugin {
  fn build(&self, app: &mut App) {
      use animation::*;
      use input::*;
      use offense::*;
      use resources::*;
      use systems::*;
      use types::*;
      use physics::*;

    // Events
    app
        .add_event::<InputEvent>()
        .add_event::<AnimationTransitionEvent>()
        .add_event::<BusyEvent>()
        .add_event::<CollisionEvent>()
        .add_event::<ImpactEvent>()
        .add_event::<CancelEvent>()
        .add_event::<LandingEvent>();

    // Resources
    app
        .insert_resource(OkizemeConfig::default())
        .insert_resource(PlayerInputSources::default())
        .insert_resource(PlayerDevices::default())
        .insert_resource(PlayerPositions::default())
        .insert_resource(PlayerHealthBars::default())
        .insert_resource(PlayerCombos::default());

    app.add_stage("main",SystemStage::single_threaded()
        .with_run_criteria(FixedTimestep::steps_per_second(60.))
        .with_system(write_inputs)
        .with_system(read_inputs.after(write_inputs))
        .with_system(manage_action_state.after(read_inputs))
        .with_system(add_busy.after(manage_action_state))
        .with_system(manage_character_velocity.after(manage_action_state))
        .with_system(apply_character_velocity.after(manage_character_velocity))
        .with_system(manage_landing.after(apply_character_velocity))
        .with_system(handle_attacks.after(apply_character_velocity))
        .with_system(detect_collisions.after(handle_attacks))
        .with_system(handle_collisions.after(detect_collisions))
        .with_system(manage_busy.after(detect_collisions))
        .with_system(manage_hitstop.after(detect_collisions))
        .with_system(manage_stun.after(detect_collisions))
    );
  }
}
