use bevy::prelude::*;

#[derive(Component)]
pub struct Blocking {
    pub block_type: BlockType,
    pub barrier: bool,
    pub age: u8,
}

impl Blocking {
    pub fn new(block_type: BlockType, barrier: bool) -> Self {
        Blocking {
            block_type,
            barrier,
            age: 0,
        }
    }
    pub fn tick(&mut self) {
        if self.age < 8 {
            self.age += 1;
        }
    }

    pub fn instant_block(&self) -> bool {
        self.age < 8
    }
}

pub enum BlockType {
    High,
    Low,
    Air,
}
