use bevy::prelude::*;
use okizeme_animation::OkiAnimationPlayer;

pub mod animation {
    pub use okizeme_animation::*;
}
pub mod input {
    pub use okizeme_input::*;
}

pub mod player {
    pub use okizeme_player::*;
}

pub mod types {
    pub use okizeme_types::*;
}

pub mod utils {
    pub use okizeme_utils::*;
}

pub mod offense {
    pub use okizeme_offense::*;
}

#[derive(Default)]
pub struct OkizemePlugin;

impl Plugin for OkizemePlugin {
    fn build(&self, app: &mut App) {
         app.register_type::<OkiAnimationPlayer>();
    }
}

