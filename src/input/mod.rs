use crate::{OkizemeGameState, OkizemePlayerSet};
use bevy::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;

mod input_buffer;
mod input_listener;
mod parsing;
mod types;

pub use input_buffer::*;
pub use input_listener::*;
pub use parsing::*;
pub use types::*;

pub struct OkiInputPlugin;

impl Plugin for OkiInputPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(OkizemePlayerSet::Input.in_set(OnUpdate(OkizemeGameState::Gameplay)));
        // Plugins
        app.add_plugin(InputManagerPlugin::<OkiAction>::default());
        // Events
        app.add_event::<InputEvent>();
        // Resources
        app.insert_resource(PlayerInputSources::default());
        // Systems
        app.add_systems(
            (publish_input_events, read_inputs)
                .chain()
                .in_set(OkizemePlayerSet::Input),
        );
    }
}
