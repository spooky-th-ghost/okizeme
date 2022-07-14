use bevy::prelude::*;
use bevy_inspector_egui::{
    WorldInspectorPlugin,
    RegisterInspectable
};
use okizeme::{
    prelude::*,
    types::{
        PlayerId,
        Busy,
        Hitstop,
        Stun
    },
    character::{
        Movement,
        ActionState
    },
    physics::Velocity
};

pub fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugin(OkizemePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<ActionState>()
        .register_inspectable::<Busy>()
        .register_inspectable::<Hitstop>()
        .register_inspectable::<Stun>();

    app
        .add_startup_system(setup)
        .run();
}


pub fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: 
            Transform::from_xyz(0., 0., 150.)
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
    });
    commands.spawn_bundle(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube{size: 10.})),
            material: materials.add(Color::ORANGE_RED.into()),
            transform: Transform::from_translation(Vec3::new(-40., 0., 0.)),
            ..default()
        })
    .insert(PlayerId::P1)
    .insert(ActionState::Idle)
    .insert(Velocity::new(Vec2::ZERO,10., true, None))
    .insert(Movement::default());

    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(40., 80.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(40., 0., 0.)),
            ..default()
        }
    )
    .insert(PlayerId::P2)
    .insert(ActionState::Idle)
    .insert(Velocity::new(Vec2::ZERO,10., true, None))
    .insert(Movement::default());
}
