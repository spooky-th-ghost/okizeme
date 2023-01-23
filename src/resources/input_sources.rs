use crate::{InputBuffer, PlayerId};
use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerInputSources(Vec<InputBuffer>);

impl Default for PlayerInputSources {
    fn default() -> Self {
        PlayerInputSources(vec![
            InputBuffer::with_player(PlayerId::P1),
            InputBuffer::with_player(PlayerId::P2),
        ])
    }
}

impl PlayerInputSources {
    pub fn get_source_mut(&mut self, player_id: &PlayerId) -> &mut InputBuffer {
        self.0
            .iter_mut()
            .find(|x| x.get_player_id() == player_id)
            .unwrap()
    }
    pub fn get_source(&self, player_id: &PlayerId) -> &InputBuffer {
        &self
            .0
            .iter()
            .find(|&x| x.get_player_id() == player_id)
            .unwrap()
    }
}
