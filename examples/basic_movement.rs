use bevy::prelude::*;
use okizeme::prelude::*;

pub fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugin(OkizemePlugin);

    app
        .add_startup_system(setup)
        .run();
}


pub fn setup (
    mut commands: Commands
) {

}
