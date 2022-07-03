use okizeme_types::PlayerId;

pub struct CharacterHealth {
    pub player_id: PlayerId,
    pub max_value: u16,
    pub current_value: u16,
    pub temp_value: u16
}

impl CharacterHealth {
    pub fn new(player_id: PlayerId) -> Self {
       CharacterHealth {
           player_id,
           max_value: 1000,
           current_value: 1000,
           temp_value: 0
       }
    }

    pub fn deal_damage(&mut self, damage: u16) {
        if self.current_value <= damage {
            self.current_value = 0;
        } else {
            self.current_value -= damage;
        }
    }
}
