use bevy::prelude::*;
use okizeme_character::{
    ActionState,
    Movement
};

use okizeme_physics::{Velocity, LandingEvent};
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
    mut player_query: Query<(&PlayerId, &mut Velocity, &mut Transform), Without<Hitstop>>,
    mut landing_writer: EventWriter<LandingEvent>
) {
    for (player_id, mut velocity, mut transform) in player_query.iter_mut() {
        let tv = velocity.get_target_velo();
        transform.translation += tv.extend(0.);
        if transform.translation.y < 0. {
            landing_writer.send(LandingEvent {player_id: *player_id});
        }
    }
}

pub fn manage_landing(
    mut player_query: Query<(&PlayerId, &mut ActionState, &mut Movement, &mut Velocity, &mut Transform)>,
    mut landing_reader: EventReader<LandingEvent>
) {
    for event in landing_reader.iter() {
        for (player_id, mut action_state, mut movement, mut velocity, mut transform) in player_query.iter_mut() {
            if event.player_id == *player_id {
                transform.translation.y = 0.;
                action_state.land();
                movement.land();
                velocity.land();
            }
        }
    }
}

