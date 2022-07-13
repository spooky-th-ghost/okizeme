use bevy::prelude::*;
use okizeme_types::{
    Hitstop,
    Stun,
    Busy,
    SelfRemoving
};

pub fn manage_hitstop(
  mut coms: Commands,
  mut query: Query<(Entity,&mut Hitstop)>,
) {
  for  (entity, mut hitstop) in query.iter_mut() {
    if hitstop.is_finished() {
        coms.entity(entity).remove::<Hitstop>();
    }
  }
}

pub fn manage_stun(
  mut coms: Commands,
  mut query: Query<(Entity,&mut Stun), Without<Hitstop>>,
) {
  for  (entity, mut stun) in query.iter_mut() {
    if stun.is_finished() {
      coms.entity(entity).remove::<Stun>();
    }
  }
}

pub fn manage_busy(
  mut coms: Commands,
  mut query: Query<(Entity,&mut Busy), Without<Hitstop>>,
) {
  for  (entity, mut busy) in query.iter_mut() {
    if busy.is_finished() {
      coms.entity(entity).remove::<Busy>();
    }
  }
}
