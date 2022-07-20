use bevy::prelude::*;
use okizeme_types::{ElementVisibility, OkizemeConfig, PlayerId};

use okizeme_defense::Hurtbox;
use okizeme_offense::Hitbox;

pub fn update_hitbox_visibility(
    config: Res<OkizemeConfig>,
    mut query: Query<(&mut Visibility, &PlayerId), With<Hitbox>>,
) {
    if config.is_changed() {
        for (mut visibility, player_id) in query.iter_mut() {
            use ElementVisibility::*;
            match config.show_hitboxes {
                Player(id) => {
                    if player_id == &id {
                        visibility.is_visible = true;
                    } else {
                        visibility.is_visible = false;
                    }
                }

                Both => {
                    visibility.is_visible = true;
                }

                Off => visibility.is_visible = false,
            }
        }
    }
}

pub fn update_hurtbox_visibility(
    config: Res<OkizemeConfig>,
    mut query: Query<(&mut Visibility, &PlayerId), With<Hurtbox>>,
) {
    if config.is_changed() {
        for (mut visibility, player_id) in query.iter_mut() {
            use ElementVisibility::*;
            match config.show_hitboxes {
                Player(id) => {
                    if player_id == &id {
                        visibility.is_visible = true;
                    } else {
                        visibility.is_visible = false;
                    }
                }

                Both => {
                    visibility.is_visible = true;
                }

                Off => visibility.is_visible = false,
            }
        }
    }
}
