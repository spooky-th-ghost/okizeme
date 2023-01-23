use crate::*;

pub struct InputBuffer {
    pub player_id: PlayerId,
    pub inputs: Vec<InputMask>,
}

impl InputBuffer {
    pub fn with_player(player_id: PlayerId) -> Self {
        InputBuffer {
            player_id,
            inputs: Vec::new(),
        }
    }

    pub fn get_player_id(&self) -> &PlayerId {
        &self.player_id
    }

    pub fn tick(&mut self) {
        if self.inputs.len() > 15 {
            self.inputs.remove(0);
        }
    }

    pub fn update(&mut self, input_event: &InputEvent) {
        self.tick();
        self.inputs.push(input_event.input_mask.clone());
    }
}
