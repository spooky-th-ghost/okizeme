use bevy::{prelude::*, time::FixedTimestep};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
fn main() {}
// use okizeme::{
//     systems::{manage_hitstop, oki_animation_player},
//     types::Hitstop,
// };
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugin(WorldInspectorPlugin)
//         .insert_resource(AmbientLight {
//             color: Color::WHITE,
//             brightness: 1.0,
//         })
//         .add_startup_system(setup)
//         .add_system(setup_scene_once_loaded)
//         .add_stage(
//             "oki",
//             SystemStage::single_threaded()
//                 .with_run_criteria(FixedTimestep::steps_per_second(60.))
//                 .with_system(oki_animation_player)
//                 .with_system(manage_hitstop),
//         )
//         .add_system(keyboard_animation_control)
//         .run();
// }

// #[derive(Resource)]
// struct Animations(Vec<Handle<AnimationClip>>);

// #[derive(Component)]
// struct Player(u8);

// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Insert a resource with the current scene information
//     commands.insert_resource(Animations(vec![
//         asset_server.load("models/Oki_frames.glb#Animation2"),
//         asset_server.load("models/Oki_frames.glb#Animation1"),
//         asset_server.load("models/Oki_frames.glb#Animation0"),
//     ]));

//     // Camera
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(0., 30.0, 150.0)
//             .looking_at(Vec3::new(0.0, 15.0, 0.0), Vec3::Y),
//         ..Default::default()
//     });

//     // Plane
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Plane { size: 500000.0 })),
//         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//         ..default()
//     });

//     // Light
//     commands.spawn(DirectionalLightBundle {
//         transform: Transform::from_rotation(Quat::from_euler(
//             EulerRot::ZYX,
//             0.0,
//             1.0,
//             -std::f32::consts::FRAC_PI_4,
//         )),
//         directional_light: DirectionalLight {
//             shadows_enabled: true,
//             ..default()
//         },
//         ..default()
//     });

//     // Oki
//     commands
//         .spawn(SceneBundle {
//             scene: asset_server.load("models/Oki_frames.glb#Scene0"),
//             ..default()
//         })
//         .insert(Player(1))
//         .insert(Name::new("Player"));
//     // .with_children(|parent| {
//     //     parent.spawn_scene(asset_server.load("models/Oki_frames.glb#Scene0"));

//     println!("Animation controls:");
//     println!("  - spacebar: Add 60 frames of hitpause");
//     println!("  - return: change animation");
// }

// // Once the scene is loaded, start the animation
// fn setup_scene_once_loaded(
//     animations: Res<Animations>,
//     mut player: Query<&mut AnimationPlayer>,
//     mut done: Local<bool>,
// ) {
//     if !*done {
//         if let Ok(mut player) = player.get_single_mut() {
//             player.play(animations.0[0].clone_weak()).repeat().pause();
//             *done = true;
//         }
//     }
// }

// fn keyboard_animation_control(
//     mut coms: Commands,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(Entity, &mut AnimationPlayer)>,
//     animations: Res<Animations>,
//     mut current_animation: Local<usize>,
// ) {
//     if let Ok((entity, mut player)) = query.get_single_mut() {
//         if keyboard_input.just_pressed(KeyCode::Return) {
//             *current_animation = (*current_animation + 1) % animations.0.len();
//             player
//                 .play(animations.0[*current_animation].clone_weak())
//                 .repeat()
//                 .pause();
//         }

//         if keyboard_input.just_pressed(KeyCode::Space) {
//             coms.entity(entity).insert(Hitstop(15));
//         }
//     }
// }
