use crate::{Combo, ComboedState, Hitbox, PlayerId};
use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerCombos(Vec<Combo>);

impl Default for PlayerCombos {
    fn default() -> Self {
        PlayerCombos(Vec::new())
    }
}

impl PlayerCombos {
    pub fn add_to_combo(
        &mut self,
        hitbox: &Hitbox,
        player_id: &PlayerId,
        comboed_state: ComboedState,
        missed_tech: bool,
    ) -> (u16, u8) {
        let existing_combo: Option<&mut Combo> =
            self.0.iter_mut().find(|c| c.player_id == *player_id);
        if let Some(combo) = existing_combo {
            combo.add_to_combo(hitbox, missed_tech, comboed_state)
        } else {
            let mut new_combo = Combo::new(player_id);
            let (damage, hitstun) = new_combo.add_initial_hit_to_combo(hitbox, comboed_state);
            self.0.push(new_combo);
            (damage, hitstun)
        }
    }
}
