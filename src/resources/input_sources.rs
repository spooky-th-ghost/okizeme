use bevy::prelude::*;

use crate::{InputMethod, InputSource, PlayerId};

#[derive(Resource)]
pub struct PlayerInputSources(Vec<InputSource>);

impl Default for PlayerInputSources {
    fn default() -> Self {
        PlayerInputSources(vec![
            InputSource::new_buffer(PlayerId::P1),
            InputSource::new_buffer(PlayerId::P2),
        ])
    }
}

impl PlayerInputSources {
    pub fn get_source_mut(&mut self, player_id: &PlayerId) -> &mut InputSource {
        self.0
            .iter_mut()
            .find(|x| x.get_player_id() == player_id)
            .unwrap()
    }
    pub fn get_source(&self, player_id: &PlayerId) -> &InputSource {
        &self
            .0
            .iter()
            .find(|&x| x.get_player_id() == player_id)
            .unwrap()
    }

    pub fn get_player_current_motion(&self, player_id: &PlayerId) -> u8 {
        if let Some(source) = self.0.iter().find(|x| x.get_player_id() == player_id) {
            source.get_current_motion()
        } else {
            5
        }
    }
}
