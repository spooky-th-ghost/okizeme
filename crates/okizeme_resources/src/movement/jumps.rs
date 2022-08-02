
pub struct Jump {
    pub jumpsquat: u8,
    pub jump_lockout: u8,
    pub jump_height: f32,
}

impl Default for Jump {
    fn default() -> Self {
        Jump {
            jumpsquat: 3,
            jump_lockout: 8,
            jump_height: 8.,
        }
    }
}
