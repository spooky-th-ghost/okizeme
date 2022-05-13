use bevy::{
    prelude::*,
    animation::AnimationPlayer
};

use okizeme_types::Freeze;

pub fn oki_animation_player(
    mut animation_players: Query<&mut AnimationPlayer,Without<Freeze>>,
) {
    for  mut player in animation_players.iter_mut() {
        let elapsed = player.elapsed();
        player.set_elapsed( elapsed + (1./60.));
    }
}

pub struct AnimationController3D {

}

pub struct AnimationMap3D;

pub struct AnimationController2D {

}

pub struct AnimationMap2D;
