use crate::character::collision::{HitboxBundle, HitboxEvent, HurtboxEvent};
use crate::types::{Busy, Frame, PlayerId};
use crate::{CharacterState, Velocity};
use bevy::ecs::system::Command;
use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use dyn_clone::DynClone;

use super::collision::Hitbox;

pub trait Attack: DynClone + Send + Sync + 'static {
    fn execute(&self, world: &mut World);
    fn startup(&self) -> u8;
    fn active(&self) -> Vec<u8>;
    fn recovery(&self) -> u8;
}

dyn_clone::clone_trait_object!(Attack);

impl Command for Box<dyn Attack> {
    fn write(self, world: &mut World) {
        self.execute(world);
    }
}

pub trait Airdash: Command + Send + Sync + 'static {
    fn lockout(&self) -> u8;
}

pub trait Dash: Command + Send + Sync + 'static {
    fn execute(&self, frame: u8, player: EntityMut);
}

#[derive(Clone)]
pub struct SingleHitbox {
    pub player_id: PlayerId,
    pub hitbox_event: HitboxEvent,
    pub hurtbox_events: Vec<HurtboxEvent>,
    pub total_duration: Frame,
    pub counter_hit_duration: Frame,
}

impl Attack for SingleHitbox {
    fn execute(&self, world: &mut World) {
        let mut my_entity: Entity = Entity::from_raw(10);
        let mut frame: u8 = 0;
        for (entity, player_id, character_state) in world
            .query::<(Entity, &PlayerId, &CharacterState)>()
            .iter(world)
        {
            if *player_id == self.player_id {
                my_entity = entity;
                frame = character_state.frame();
            }
        }
        let mut player = world.entity_mut(my_entity);

        if !player.contains::<Attacking>() {
            player.insert(Attacking);
        }

        if frame == self.hitbox_event.frame {
            player.with_children(|parent| {
                parent.spawn(HitboxBundle::new(
                    self.player_id,
                    self.hitbox_event.hitbox,
                    self.hitbox_event.position,
                    self.hitbox_event.size,
                ));
            });
        }
    }
    fn startup(&self) -> u8 {
        self.hitbox_event.frame
    }

    fn active(&self) -> Vec<u8> {
        vec![self.hitbox_event.hitbox.duration.get()]
    }

    fn recovery(&self) -> u8 {
        let active_sum: u8 = self.active().iter().sum();
        self.total_duration.get() - self.startup() - active_sum
    }
}

#[derive(Component)]
pub struct Attacking;
