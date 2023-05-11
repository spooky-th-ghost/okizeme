use crate::character::actions::{Action, Attacking};
use crate::character::collision::{HitboxBundle, HitboxEvent, HurtboxEvent};
use crate::{CharacterState, Frame, PlayerId};
use bevy::prelude::*;
#[derive(Clone, Default)]
pub struct SingleHitbox {
    pub player_id: PlayerId,
    pub hitbox_event: HitboxEvent,
    pub hurtbox_events: Vec<HurtboxEvent>,
    pub total_duration: Frame,
    pub counter_hit_duration: Frame,
}

impl Action for SingleHitbox {
    fn execute(&self, world: &mut World) {
        let mut my_entity: Option<Entity> = None;
        let mut frame: u8 = 0;
        for (entity, player_id, character_state) in world
            .query::<(Entity, &PlayerId, &CharacterState)>()
            .iter(world)
        {
            if *player_id == self.player_id {
                my_entity = Some(entity);
                frame = character_state.frame();
            }
        }
        if let Some(entity) = my_entity {
            let mut player = world.entity_mut(entity);

            if !player.contains::<Attacking>() {
                player.insert(Attacking);
            }

            if frame == self.hitbox_event.frame.get() {
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
    }
    fn startup(&self) -> Vec<u8> {
        vec![self.hitbox_event.frame.get()]
    }

    fn active(&self) -> Vec<u8> {
        vec![self.hitbox_event.hitbox.duration.get()]
    }

    fn recovery(&self) -> u8 {
        let active_sum: u8 = self.active().iter().sum();
        self.total_duration.get() - self.startup()[0] - active_sum
    }
}

#[derive(Default)]
pub struct Attack {
    pub player_id: Option<PlayerId>,
    pub hitbox_events: Vec<HitboxEvent>,
    pub hurtbox_events: Vec<HurtboxEvent>,
    // pub transform_events: Vec<>,
    // pub velocity_events: Vec<>
}

impl Attack {
    pub fn new() -> Self {
        Attack::default()
    }

    pub fn with_hitbox(mut self, hitbox_event: HitboxEvent) -> Self {
        self.hitbox_events.push(hitbox_event);
        self
    }

    pub fn with_hurtbox(mut self, hurtbox_event: HurtboxEvent) -> Self {
        self.hurtbox_events.push(hurtbox_event);
        self
    }
}
