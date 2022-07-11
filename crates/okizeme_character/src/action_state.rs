use bevy::prelude::*;
use okizeme_input::{
    InputSource,
    CommandType,
    InputMethod
};
use okizeme_animation::AnimationTransition;
use okizeme_physics::{
    Velocity,
    InterpolatedForce
};

use crate::{
    Movement,
    Backdash
};

/// Handles the current state of the player
#[derive(Default, Debug, Clone, Copy, Component)]
pub enum ActionState {
    #[default]
    Idle,
    Walking,
    BackWalking,
    Attacking,
    AttackingAirborne,
    Recovering,
    RecoveringAirborne,
    Crouching,
    Jumpsquat {velocity: Vec2},
    AirJumpsquat {velocity: Vec2},
    Rising,
    Falling,
    Dashing,
    BackDashing,
    AirDashing {duration: u8, velocity: Vec2},
    AirBackDashing {duration: u8, velocity: Vec2}
}

impl PartialEq for ActionState {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl ActionState {
    pub fn update (
        &mut self,
        input: &InputSource,
        position: Vec3,
        movement: &mut Movement, 
        velocity: &mut Velocity,
    ) -> (Option<AnimationTransition>, u8){
        use ActionState::*;

        let (new_state, busy): (ActionState, u8) = match *self {
            Idle | Walking | BackWalking | BackDashing => self.from_neutral_states(input, movement, velocity),
            Dashing => self.from_dashing(input, movement),
            Rising 
            | Falling 
            | AirDashing { duration:_, velocity:_ }
            | AirBackDashing { duration:_, velocity:_ } => self.from_neutral_airborne(input, movement, velocity, position),
            _ => (*self, 0)
        };

        let possible_transition = self.calculate_transition(new_state);
        *self = new_state;
        (possible_transition, busy)
    }

    pub fn from_neutral_states (
        &self,
        input: &InputSource,
        movement: &mut Movement,
        velocity: &mut Velocity
    ) -> (Self, u8) {
        use ActionState::*;

        if let Some(ct) = input.get_command_type() {
            match ct {
                CommandType::Dash => return (Dashing, 0),
                CommandType::BackDash => return self.buffer_backdash(movement, velocity),
                _ => ()
            }
        }

        match input.get_current_motion() {
            4 => (BackWalking, 0),
            6 => (Walking, 0),
            1 | 2 | 3 => (Crouching, 0),
            7 | 8 | 9 => Self::buffer_jump(input.get_current_motion(), movement, false),
            _ => (Idle, 0)
        }
    }

    /// Returns a new state based on input from the following states:
    ///  - Rising
    ///  - Falling
    ///  - Airdashing
    ///  - Airbackdashing
    pub fn from_neutral_airborne(&self, buffer: &InputSource, movement: &mut Movement, velocity: &mut Velocity, position: Vec3) -> (Self, u8) {
        use ActionState::*;
            if position.y <= 0.0 {
            return (Idle, 0);
        }
        match self {
            Rising 
            | Falling 
            | AirDashing { duration: _, velocity:_} 
            | AirBackDashing { duration: _, velocity:_} => {
                self.from_airborne_input(buffer, movement, velocity)
            },
            _ => (*self, 0)
        }
    }
    /// Returns a new state based on input from dashing
    pub fn from_dashing(
        &self,
        input: &InputSource,
        movement: &Movement
    ) -> (Self, u8) {
        use ActionState::*;
        match input.get_current_motion() {
            4 => (BackWalking, 0),
            6 => (Dashing, 0),
            1 | 2 | 3 => (Crouching, 0),
            7 | 8 | 9 => Self::buffer_dash_jump(input.get_current_motion(), movement, false),
            _ => (Idle, 0)
        }
    }

    pub fn from_jump_squat(&self, movement: &Movement, velocity: &mut Velocity) -> (Self, u8) {
        use ActionState::*;
        match self {
            Jumpsquat { velocity: jump_velocity } => {
                velocity.force = *jump_velocity;
                (Rising, movement.jump_lockout)
            },
            _ => (*self, 0)
        }
    }

    /// Returns a new state from input while aireborne
    pub fn from_airborne_input(
        &self,
        buffer: &InputSource,
        movement: &mut Movement,
        velocity: &mut Velocity
    ) -> (Self, u8) {
        use ActionState::*;

        if movement.can_airdash() {
            if let Some(ct) = buffer.get_command_type() {
                match ct {
                    CommandType::Dash => {
                        movement.spend_airdash();
                        return self.buffer_airdash(movement, true)
                    },
                    CommandType::BackDash => {
                        movement.spend_airdash();
                        return self.buffer_airdash(movement, false)
                    },
                    _ => ()
                }
            }
        }

        match self {
            AirDashing { duration:_, velocity:_} 
            | AirBackDashing {duration:_, velocity:_ }=> {
                if self.is_finished_airdashing() {
                    (Falling, 0)
                } else {
                    (*self, 0)
                }
            },
            Rising => {
                if velocity.is_falling() {
                    (Falling, 0)
                } else {
                    (*self, 0)
                }
            }
            _ => (*self, 0)
        }
    }

    /// Updates the current state if finishing an airdash
    pub fn is_finished_airdashing(&self) -> bool {
        use ActionState::*;
        match self {
            AirDashing {duration, velocity:_} => *duration == 0,
            AirBackDashing {duration, velocity:_} => *duration == 0,
            _ => false,
        }
    }

    /// If the new state does not match the old state, generate an animation transition
    fn calculate_transition(&self, other: Self) -> Option<AnimationTransition> {
        use ActionState::*;
        use AnimationTransition::*;
        match (self, other) {
            (Rising, Falling) => Some(RiseToFall),
            (Falling, Idle) | (Rising, Idle) => Some(FallToIdle),
            (Crouching,Idle) => Some(CrouchToIdle),
            (Walking,Idle) => Some(WalkToIdle),
            (BackWalking,Idle) => Some(BackwalkToIdle),
            (Dashing,Idle) => Some(DashToIdle),
            (BackDashing,Idle) => Some(BackDashToIdle),
            (AirDashing { duration:_, velocity:_}, Falling) => Some(AirdashToFall),
            (AirBackDashing { duration:_, velocity:_}, Falling) => Some(AirbackdashToFall),
            (_, Idle) => Some(ToIdle),
            (_, Jumpsquat { velocity:_}) => Some(ToRise),
            (_, Walking) => Some(ToWalk),
            (_, BackWalking) => Some(ToBackwalk),
            (_, Dashing) => Some(ToDash),
            (_, BackDashing) => Some(ToBackdash),
            (_, AirDashing { duration:_, velocity:_}) => Some(ToAirdash),
            (_, AirBackDashing { duration:_, velocity:_}) => Some(ToAirBackdash),
            (_, Crouching) => Some(ToCrouch),
            (_,_) => None
        }
    }

    /// Returns a Jumpsquat state from a neutral state, with a buffered jump based on character movement and input buffer
    fn buffer_jump(motion:u8, movement: &Movement, superjump: bool) -> (ActionState, u8) {

        let x_velocity = match motion {
            7 => movement.facing_vector * (-movement.back_walk_speed*1.75),
            9 => movement.facing_vector * (movement.walk_speed),
            _ => 0.0
        };

        let y_velocity = if superjump {
            movement.jump_height * 1.25
        } else {
            movement.jump_height
        };

        let jump_startup = if superjump {
            5
        } else {
            3
        };

        let velocity = Vec2::new(x_velocity, y_velocity);
        (ActionState::Jumpsquat {velocity}, jump_startup)
    }

    /// Returns a Jumpsquat state from a Dash state, with a buffered jump based on character movement and input buffer
    fn buffer_dash_jump(
        motion: u8,
        movement: &Movement,
        superjump: bool
    ) -> (Self, u8) {
        let x_velocity = match motion {
            7 => movement.facing_vector * (-movement.back_walk_speed),
            9 => movement.facing_vector * (movement.walk_speed * 2.0),
            _ => movement.facing_vector * (movement.walk_speed * 0.5)
        };

        let y_velocity = if superjump {
            movement.jump_height * 1.25
        } else {
            movement.jump_height
        };

        let jump_startup = if superjump {
            5
        } else {
            3
        };

        let velocity = Vec2::new(x_velocity, y_velocity);
        (Self::Jumpsquat {velocity}, jump_startup)
    }

    fn buffer_airdash(&self, movement: &mut Movement, forward: bool) -> (Self, u8) {
        use ActionState::*;
        if forward {
            (AirDashing {duration: movement.max_airdash_time, velocity: Vec2::X * movement.air_dash_speed * movement.facing_vector}, movement.air_dash_lockout)
        } else {
            (AirBackDashing {duration: movement.max_air_backdash_time, velocity: Vec2::X * movement.air_dash_speed * -movement.facing_vector }, movement.air_dash_lockout)
        }
    }

    /// Returns a backdashing state, based on movement
    fn buffer_backdash(&self, movement: &mut Movement, velocity: &mut Velocity) -> (Self, u8) {
        use ActionState::*;
        use Backdash::*;

        match movement.backdash {
            Standard {speed, busy, motion_duration} => {
                let i_force = InterpolatedForce::new(
                    Vec2::new(-speed * movement.facing_vector, 0.0),
                    Vec2::new(-2.0 * movement.facing_vector, 0.0),
                    motion_duration
                );
                velocity.set_interpolated_force(i_force);
            (BackDashing, busy)
            },
            _ => (Self::Idle, 0)
    }
  }
}
