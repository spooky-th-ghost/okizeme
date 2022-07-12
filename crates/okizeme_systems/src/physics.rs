use bevy::prelude::*;
use okizeme_character::{
    ActionState,
    Movement
};

use okizeme_physics::Velocity;
use okizeme_resources::PlayerPositions;
use okizeme_types::{
    Hitstop,
    PlayerId
};

pub fn manage_character_velocity (
    mut player_query: Query<(&PlayerId, &ActionState, &Movement, &mut Velocity), Without<Hitstop>>,
    player_positions: Res<PlayerPositions>
) {
    for (player_id, state, movement, mut velocity) in player_query.iter_mut() {
        use ActionState::*;
        let facing_vector = player_positions.get_facing_vector(player_id);
        let force = match *state {
            Walking => Vec2::X * facing_vector * movement.walk_speed,
            BackWalking => Vec2::X * -facing_vector * movement.back_walk_speed,
            Rising | Falling => velocity.force - (Vec2::Y * movement.gravity),
            Dashing => Vec2::X * facing_vector * movement.dash_speed,
            BackDashing => Vec2::ZERO,
            AirDashing { duration:_, velocity: dash_velo }
            | AirBackDashing { duration:_, velocity: dash_velo } => dash_velo,
            _ => velocity.force.lerp(Vec2::ZERO, 0.5)
        };

        velocity.force = force;
    }
}

pub fn apply_character_velocity(
    mut player_query: Query<(&mut Velocity, &mut Transform), Without<Hitstop>>
) {
    for (mut velocity, mut transform) in player_query.iter_mut() {
        let tv = velocity.get_target_velo();
        transform.translation += tv.extend(0.);
    }
}
