use bevy::prelude::*;

use crate::{
  Velocity
};

pub fn apply_velocity(
  mut query: Query<(&mut Velocity, &mut Transform)>
) {
  for (velocity, mut transform) in query.iter_mut() {
    transform.translation += velocity.force.extend(0.);
    if transform.translation.y < 0.0 && velocity.collides_with_ground {
      transform.translation.y = 0.0;
    }
  }
}
