use bevy::{prelude::*, time::FixedTimestep};
#[macro_use]
extern crate lazy_static;

pub mod animation;
pub mod character;
pub mod components;
pub mod defense;
pub mod events;
pub mod input;
pub mod macros;
pub mod offense;
pub mod physics;
pub mod resources;
pub mod systems;
pub mod types;
pub mod utils;

#[doc(hidden)]
pub use animation::*;
#[doc(hidden)]
pub use character::*;
#[doc(hidden)]
pub use components::*;
#[doc(hidden)]
pub use defense::*;
#[doc(hidden)]
pub use events::*;
#[doc(hidden)]
pub use input::*;
#[doc(hidden)]
pub use macros::*;
#[doc(hidden)]
pub use offense::*;
#[doc(hidden)]
pub use physics::*;
#[doc(hidden)]
pub use resources::*;
#[doc(hidden)]
pub use systems::*;
#[doc(hidden)]
pub use types::*;
#[doc(hidden)]
pub use utils::*;

pub mod prelude {
    pub use crate::OkizemePlugin;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, SystemLabel)]
pub enum OkizemeSystemLabels {
    InputPhase,
    ActionPhase,
    CollisionPhase,
    PhysicsPhase,
    ResultPhase,
    CleanupPhase,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OkizemeStates {
    Gameplay,
    ResultsScreen,
    TrainingMenu,
    CharacterSelect,
    MainMenu,
}

pub struct OkizemePlugin;

impl Plugin for OkizemePlugin {
    fn build(&self, app: &mut App) {
        use OkizemeSystemLabels::*;

        // Events
        app.add_event::<InputEvent>()
            .add_event::<AnimationTransitionEvent>()
            .add_event::<BusyEvent>()
            .add_event::<CollisionEvent>()
            .add_event::<ImpactEvent>()
            .add_event::<CancelEvent>()
            .add_event::<LandingEvent>();

        // Resources
        app.insert_resource(OkizemeConfig::default())
            .insert_resource(PlayerInputSources::default())
            .insert_resource(PlayerDevices::default())
            .insert_resource(PlayerPositions::default())
            .insert_resource(PlayerHealthBars::default())
            .insert_resource(PlayerCombos::default());
        // State
        app.add_state(OkizemeStates::Gameplay);

        // Systems
        app.add_stage(
            "main",
            SystemStage::single_threaded()
                // Ensure that everything runs at 60 frames per second
                .with_run_criteria(FixedTimestep::steps_per_second(60.))
                // Input Phase
                .with_system_set(
                    SystemSet::on_update(OkizemeStates::Gameplay)
                        .label(InputPhase)
                        .with_system(write_inputs_to_buffer)
                        .with_system(read_inputs.after(write_inputs_to_buffer)),
                )
                // Action Phase
                .with_system_set(
                    SystemSet::on_update(OkizemeStates::Gameplay)
                        .label(ActionPhase)
                        .after(InputPhase)
                        .with_system(manage_action_state)
                        .with_system(add_busy.after(manage_action_state))
                        .with_system(handle_attacks),
                )
                // Collision Phase
                .with_system_set(
                    SystemSet::on_update(OkizemeStates::Gameplay)
                        .label(CollisionPhase)
                        .after(ActionPhase)
                        .with_system(detect_collisions)
                        .with_system(handle_collisions.after(detect_collisions)), // .with_system(manage_busy.after(detect_collisions))
                )
                // Physics Phase
                .with_system_set(
                    SystemSet::on_update(OkizemeStates::Gameplay)
                        .label(PhysicsPhase)
                        .after(CollisionPhase)
                        .with_system(manage_character_velocity)
                        .with_system(apply_character_velocity.after(manage_character_velocity))
                        .with_system(manage_landing.after(apply_character_velocity)),
                )
                // Results Phase
                .with_system_set(
                    SystemSet::on_update(OkizemeStates::Gameplay)
                        .label(ResultPhase)
                        .after(PhysicsPhase),
                )
                // Cleanup Phase
                .with_system_set(
                    SystemSet::on_update(OkizemeStates::Gameplay)
                        .label(CleanupPhase)
                        .after(ResultPhase)
                        .with_system(manage_busy)
                        .with_system(manage_hitstop)
                        .with_system(manage_stun),
                ), // .with_system(write_inputs)
                   // .with_system(read_inputs.after(write_inputs))
                   // .with_system(manage_action_state.after(read_inputs))
                   // .with_system(add_busy.after(manage_action_state))
                   // .with_system(manage_character_velocity.after(manage_action_state))
                   // .with_system(apply_character_velocity.after(manage_character_velocity))
                   // .with_system(manage_landing.after(apply_character_velocity))
                   // .with_system(handle_attacks.after(apply_character_velocity))
                   // .with_system(detect_collisions.after(handle_attacks))
                   // .with_system(handle_collisions.after(detect_collisions)), // .with_system(manage_busy.after(detect_collisions))
                   // .with_system(manage_hitstop.after(detect_collisions))
                   // .with_system(manage_stun.after(detect_collisions)),
        );
    }
}
