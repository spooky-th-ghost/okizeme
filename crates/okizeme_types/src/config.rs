use crate::PlayerId;

#[derive(Clone, Copy)]
pub enum ElementVisibility {
    Player(PlayerId),
    Both,
    Off,
}

pub struct OkizemeConfig {
    pub show_hitboxes: ElementVisibility,
    pub show_hurtboxes: ElementVisibility,
    pub show_inputs: ElementVisibility,
    pub show_meter: ElementVisibility,
    pub show_health: ElementVisibility,
    pub show_move_data: ElementVisibility,
    pub show_damage: ElementVisibility,
}

impl OkizemeConfig {
    pub fn get_hitbox_visibility(&self, player_id: &PlayerId) -> bool {
        use ElementVisibility::*;
        match self.show_hitboxes {
            Player(pid) => pid == *player_id,
            Both => true,
            Off => false,
        }
    }
}

impl Default for OkizemeConfig {
    fn default() -> Self {
        let training_element_visibility = if cfg!(feature = "debug") {
            ElementVisibility::Both
        } else {
            ElementVisibility::Off
        };

        OkizemeConfig {
            show_hitboxes: training_element_visibility,
            show_hurtboxes: training_element_visibility,
            show_inputs: training_element_visibility,
            show_move_data: training_element_visibility,
            show_meter: ElementVisibility::Both,
            show_health: ElementVisibility::Both,
            show_damage: ElementVisibility::Both,
        }
    }
}
