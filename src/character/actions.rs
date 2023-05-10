use bevy::ecs::system::Command;
use bevy::prelude::*;
use dyn_clone::DynClone;

pub trait Action: DynClone + Send + Sync + 'static {
    fn execute(&self, world: &mut World);
    fn startup(&self) -> Vec<u8>;
    fn active(&self) -> Vec<u8>;
    fn recovery(&self) -> u8;
}

dyn_clone::clone_trait_object!(Action);

impl Command for Box<dyn Action> {
    fn write(self, world: &mut World) {
        self.execute(world);
    }
}

#[derive(Component)]
pub struct Attacking;
