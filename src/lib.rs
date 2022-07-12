pub use bevy::{
    core::FixedTimestep,
    prelude::*
};
pub use okizeme_core::{
    animation::AnimationTransitionEvent,
    offense::{
        CollisionEvent,
        ImpactEvent,
        CancelEvent
    },
    types::{
        OkizemeConfig,
        BusyEvent,
        manage_stun,
        manage_busy,
        manage_hitstop
    },
    player::{
        PlayerInputSources,
        PlayerDevices,
        PlayerPositions,
        PlayerCombos,
        PlayerHealthBars,
    },
    input::InputEvent,
    systems::{
        write_inputs,
        read_inputs,
        manage_action_state,
        add_busy,
        manage_character_velocity,
        apply_character_velocity,
        handle_attacks,
        detect_collisions,
        handle_collisions
    }
};

pub struct OkizemePlugin;

impl Plugin for OkizemePlugin {
  fn build(&self, app: &mut App) {
    // Events
    app
        .add_event::<InputEvent>()
        .add_event::<AnimationTransitionEvent>()
        .add_event::<BusyEvent>()
        .add_event::<CollisionEvent>()
        .add_event::<ImpactEvent>()
        .add_event::<CancelEvent>();

    // Resources
    app
        .insert_resource(OkizemeConfig::default())
        .insert_resource(PlayerInputSources::default())
        .insert_resource(PlayerDevices::default())
        .insert_resource(PlayerPositions::default())
        .insert_resource(PlayerHealthBars::default())
        .insert_resource(PlayerCombos::default());

    app.add_stage("main",SystemStage::single_threaded()
        .with_run_criteria(FixedTimestep::steps_per_second(60.0))
        .with_system(write_inputs)
        .with_system(read_inputs.after(write_inputs))
        .with_system(manage_action_state.after(read_inputs))
        .with_system(manage_character_velocity.after(manage_action_state))
        .with_system(apply_character_velocity.after(manage_character_velocity))
        .with_system(handle_attacks.after(apply_character_velocity))
        .with_system(detect_collisions.after(handle_attacks))
        .with_system(handle_collisions.after(detect_collisions))

        );
        // .with_system(manage_character_velocity.after(manage_action_state));
        // .with_system(
        //   apply_character_velocity
        //     .label(FighterSystemLabels::PhysicsExecute)
        //     .after(FighterSystemLabels::PhysicsUpdate)
        // )
        // .with_system(
        //   read_animation_transitions
        //     .label(FighterSystemLabels::AnimationUpdate)
        //     .after(FighterSystemLabels::PhysicsExecute)
        // )
        // .with_system(
        //   animate_sprite_system
        //     .label(FighterSystemLabels::AnimationExecute)
        //     .after(FighterSystemLabels::AnimationUpdate)
        // )
        // .with_system(
        //   manage_hitboxes
        //     .label(FighterSystemLabels::HitboxUpdate)
        //     .after(FighterSystemLabels::AnimationUpdate)
        // )
        // .with_system(
        //   spawn_hitboxes
        //     .label(FighterSystemLabels::HitboxCreation)
        //     .after(FighterSystemLabels::HitboxUpdate)
        // )
      // );
  }
}
