use bevy::prelude::*;
use okizeme::execute_character_actions;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(execute_character_actions)
        .run();
}
