use bevy::prelude::*;
use okizeme_types::PlayerId;


pub struct LandingEvent {
    pub player_id: PlayerId
}
/// Component used to move transforms
#[derive(Component)]
pub struct Velocity {
  pub force: Vec2,
  pub gravity: f32,
  pub collides_with_ground: bool,
  interpolated_force: Option<InterpolatedForce>
}

impl Velocity {
    pub fn new(force: Vec2, gravity: f32, collides_with_ground: bool, interpolated_force: Option<InterpolatedForce>) -> Self {
        Velocity {
            force,
            gravity,
            collides_with_ground,
            interpolated_force
        }
    }
    pub fn interpolated_force(&self) -> Option<InterpolatedForce> {
        self.interpolated_force
    }

    pub fn set_interpolated_force(&mut self, interpolated_force: InterpolatedForce) {
        self.interpolated_force = Some(interpolated_force);
    }

    pub fn is_falling(&self) -> bool {
      self.force.y < 0.0
    }

    pub fn get_target_velo(&mut self) -> Vec2 {
        if let Some(i_force) = self.interpolated_force.as_mut() {
            i_force.update();
            let i_force_velo = i_force.current_velocity;
            if i_force.is_finished() {self.interpolated_force = None;}
            i_force_velo + self.force
        } else {
            self.force
        }
    }

    pub fn land(&mut self) {
        self.force.y = 0.;
        self.interpolated_force = None;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InterpolatedForce {
  current_velocity: Vec2,
  ending_velocity: Vec2,
  duration: u8,
  frames_elapsed: u8
}

impl InterpolatedForce {

  pub fn new(current_velocity: Vec2, ending_velocity: Vec2, duration: u8) -> Self {
    InterpolatedForce {
      current_velocity,
      ending_velocity,
      duration,
      frames_elapsed: 0
    }
  }

  pub fn update(&mut self) {
    self.tick();
    let t = self.frames_elapsed as f32 / self.duration as f32;
    self.current_velocity = self.current_velocity.lerp(self.ending_velocity,t);
  }

  pub fn tick(&mut self) {
    self.frames_elapsed += 1;
  }

  pub fn is_finished(&self) -> bool {
    self.duration == self.frames_elapsed
  }
}
