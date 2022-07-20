use okizeme_defense::CharacterHealth;
use okizeme_types::PlayerId;

pub struct PlayerHealthBars(Vec<CharacterHealth>);

impl PlayerHealthBars {
    pub fn get_health(&self, player_id: &PlayerId) -> u16 {
        self.0
            .iter()
            .find(|x| x.player_id == *player_id)
            .unwrap()
            .current_value
    }

    pub fn get_health_percentage(&self, player_id: &PlayerId) -> f32 {
        let healthbar = self.0.iter().find(|x| x.player_id == *player_id).unwrap();
        healthbar.current_value as f32 / healthbar.max_value as f32
    }

    pub fn deal_damage(&mut self, player_id: &PlayerId, damage: u16) {
        self.0
            .iter_mut()
            .find(|x| x.player_id == *player_id)
            .unwrap()
            .deal_damage(damage);
    }
}

impl Default for PlayerHealthBars {
    fn default() -> Self {
        PlayerHealthBars(vec![
            CharacterHealth::new(PlayerId::P1),
            CharacterHealth::new(PlayerId::P2),
        ])
    }
}
