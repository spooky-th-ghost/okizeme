use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use okizeme_animation::AnimationTransition;
use okizeme_input::{CommandType, InputMethod, InputSource};
use okizeme_physics::{InterpolatedForce, Velocity};
use okizeme_utils::*;

use crate::{Backdash, Movement};

/// Handles the current state of a character
#[derive(Default, Debug, Clone, Component, Inspectable)]
pub enum CharacterState {
    #[default]
    Idle,
    Walking,
    BackWalking,
    Attacking,
    AttackingAirborne,
    Recovering,
    RecoveringAirborne,
    Crouching,
    Jumpsquat {
        ///The number of frames until the action completes naturally
        duration: u8,
        ///The Velocity of the buffered jump
        velocity: Vec2,
    },
    AirJumpsquat {
        ///The number of frames until the action completes naturally
        duration: u8,
        ///The velocity of the buffered jump  
        velocity: Vec2,
    },
    Rising,
    Falling,
    Juggle,
    Dashing,
    BackDashing {
        ///The number of frames until the action completes naturally
        duration: u8,
    },
    AirDashing {
        ///The number of frames until the action completes naturally
        duration: u8,
        ///The velocity of the air dash
        velocity: Vec2,
    },
    AirBackDashing {
        ///The number of frames until the action completes naturally
        duration: u8,
        ///The velocity of the air dash
        velocity: Vec2,
    },
}

impl PartialEq for CharacterState {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl CharacterState {
    /// Advance the duration of the state if applicable
    fn tick(&mut self) {
        use CharacterState::*;
        match self {
            Jumpsquat {
                duration,
                velocity: _,
            } => {
                *duration = countdown(*duration);
            }
            AirJumpsquat {
                duration,
                velocity: _,
            } => {
                *duration = countdown(*duration);
            }
            BackDashing { duration } => {
                *duration = countdown(*duration);
            }
            AirDashing {
                duration,
                velocity: _,
            } => {
                *duration = countdown(*duration);
            }
            AirBackDashing {
                duration,
                velocity: _,
            } => {
                *duration = countdown(*duration);
            }
            _ => (),
        }
    }

    /// updates a character state, advancing it's timers and changing it based on input and character movement
    pub fn update(
        &mut self,
        buffer: &InputSource,
        movement: &mut Movement,
        velocity: &mut Velocity,
        position: Vec3,
    ) -> Option<AnimationTransition> {
        use CharacterState::*;
        self.tick();

        let new_state = match self {
            Idle | Walking | BackWalking | Crouching => {
                self.from_neutral_states(buffer, movement, velocity)
            }
            Dashing => self.from_dashing(buffer, movement),
            Jumpsquat {
                duration: _,
                velocity: _,
            } => self.from_jump_squat(velocity),
            Rising | Falling => self.from_neutral_airborne(buffer, movement, velocity, position),
            BackDashing { duration: _ } => self.from_backdashing(buffer, movement, velocity),
            AirDashing {
                duration: _,
                velocity: _,
            }
            | AirBackDashing {
                duration: _,
                velocity: _,
            } => self.from_air_dashing(buffer, movement, velocity),
            _ => self.clone(),
        };
        let transition = self.calculate_transition(&new_state);
        // let transition = if self.clone() != new_state {
        //     self.calculate_transition(&new_state)
        // } else {
        //     None
        // };
        *self = new_state;
        transition
    }

    /// Returns a new state based on input from the following states:
    ///  - Idle
    ///  - Walking
    ///  - Backwalking
    ///  - Crouching
    pub fn from_neutral_states(
        &self,
        buffer: &InputSource,
        movement: &mut Movement,
        velocity: &mut Velocity,
    ) -> Self {
        use CharacterState::*;

        if let Some(ct) = buffer.get_command_type() {
            match ct {
                CommandType::Dash => return Dashing,
                CommandType::BackDash => return self.buffer_backdash(movement, velocity),
                _ => (),
            }
        }

        match buffer.get_current_motion() {
            4 => BackWalking,
            6 => Walking,
            1 | 2 | 3 => Crouching,
            7 | 8 | 9 => Self::buffer_jump(buffer.get_current_motion(), &movement.clone(), false),
            _ => Idle,
        }
    }

    /// Returns a new state based on the current state when in jump squat
    pub fn from_jump_squat(&self, velocity: &mut Velocity) -> Self {
        use CharacterState::*;
        match self {
            Jumpsquat {
                duration,
                velocity: j_velocity,
            } => {
                if *duration == 0 {
                    velocity.force = *j_velocity;
                    Rising //TODO add a busy
                } else {
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }

    /// Returns a new state based on input from dashing
    pub fn from_dashing(&self, buffer: &InputSource, movement: &Movement) -> Self {
        use CharacterState::*;
        match buffer.get_current_motion() {
            4 => BackWalking,
            6 => Dashing,
            1 | 2 | 3 => Crouching,
            7 | 8 | 9 => Self::buffer_dash_jump(buffer.get_current_motion(), movement, false),
            _ => Idle,
        }
    }

    /// Returns a new state based on input from air dashing
    pub fn from_air_dashing(
        &self,
        buffer: &InputSource,
        movement: &mut Movement,
        velocity: &mut Velocity,
    ) -> Self {
        use CharacterState::*;
        match self {
            AirDashing {
                duration,
                velocity: _,
            } => {
                if *duration == 0 {
                    return self.from_neutral_airborne(buffer, movement, velocity, Vec3::ONE);
                }
                self.clone()
            }
            AirBackDashing {
                duration,
                velocity: _,
            } => {
                if *duration == 0 {
                    return self.from_neutral_airborne(buffer, movement, velocity, Vec3::ONE);
                }
                self.clone()
            }
            _ => self.clone(),
        }
    }

    /// Returns a new state based on input from the following states:
    ///  - Rising
    ///  - Falling
    ///  - Airdashing
    ///  - Airbackdashing
    pub fn from_neutral_airborne(
        &self,
        buffer: &InputSource,
        movement: &mut Movement,
        velocity: &mut Velocity,
        position: Vec3,
    ) -> Self {
        use CharacterState::*;
        if position.y <= 0.0 {
            return Idle;
        }
        match self {
            Rising
            | Falling
            | AirDashing {
                duration: _,
                velocity: _,
            }
            | AirBackDashing {
                duration: _,
                velocity: _,
            } => self.from_airborne_input(buffer, movement, velocity),
            _ => self.clone(),
        }
    }

    /// Returns a new state based on input and the backdash timer from backdash
    pub fn from_backdashing(
        &self,
        buffer: &InputSource,
        movement: &mut Movement,
        velocity: &mut Velocity,
    ) -> Self {
        use CharacterState::*;
        match self {
            BackDashing { duration } => {
                if *duration == 0 {
                    return self.from_neutral_states(buffer, movement, velocity);
                }
                self.clone()
            }
            _ => self.clone(),
        }
    }

    /// Returns a new state from input while aireborne
    pub fn from_airborne_input(
        &self,
        buffer: &InputSource,
        movement: &mut Movement,
        velocity: &mut Velocity,
    ) -> Self {
        use CharacterState::*;

        if movement.can_airdash() {
            if let Some(ct) = buffer.get_command_type() {
                match ct {
                    CommandType::Dash => {
                        movement.spend_airdash();
                        return self.buffer_airdash(movement, true);
                    }
                    CommandType::BackDash => {
                        movement.spend_airdash();
                        return self.buffer_airdash(movement, false);
                    }
                    _ => (),
                }
            }
        }

        match self {
            AirDashing {
                duration: _,
                velocity: _,
            } => {
                if self.is_finished_airdashing() {
                    Falling
                } else {
                    self.clone()
                }
            }
            AirBackDashing {
                duration: _,
                velocity: _,
            } => {
                if self.is_finished_airdashing() {
                    Falling
                } else {
                    self.clone()
                }
            }
            Rising => {
                if velocity.is_falling() {
                    Falling
                } else {
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }

    /// Returns a backdashing state, based on movement
    fn buffer_backdash(&self, movement: &mut Movement, velocity: &mut Velocity) -> Self {
        use Backdash::*;
        match movement.backdash {
            Standard {
                speed,
                busy,
                motion_duration,
            } => {
                let i_force = InterpolatedForce::new(
                    Vec2::new(-speed * movement.facing_vector, 0.0),
                    Vec2::new(-2.0 * movement.facing_vector, 0.0),
                    motion_duration,
                );
                velocity.set_interpolated_force(i_force);
                CharacterState::BackDashing { duration: busy } //TODO add a busy here
            }
            _ => Self::Idle,
        }
    }

    fn buffer_airdash(&self, movement: &mut Movement, forward: bool) -> Self {
        use CharacterState::*;
        if forward {
            // TODO: add busy here
            AirDashing {
                duration: movement.max_airdash_time,
                velocity: Vec2::X * movement.air_dash_speed * movement.facing_vector,
            }
        } else {
            AirBackDashing {
                duration: movement.max_air_backdash_time,
                velocity: Vec2::X * movement.air_dash_speed * -movement.facing_vector,
            }
        }
    }

    /// Returns a Jumpsquat state from a Dash state, with a buffered jump based on character movement and input buffer
    fn buffer_dash_jump(motion: u8, movement: &Movement, superjump: bool) -> Self {
        let x_velocity = match motion {
            7 => movement.facing_vector * (-movement.back_walk_speed),
            9 => movement.facing_vector * (movement.walk_speed * 2.0),
            _ => movement.facing_vector * (movement.walk_speed * 0.5),
        };

        let y_velocity = if superjump {
            movement.jump_height * 1.25
        } else {
            movement.jump_height
        };

        let velocity = Vec2::new(x_velocity, y_velocity);
        Self::Jumpsquat {
            duration: 3,
            velocity,
        }
    }

    /// Returns a Jumpsquat state from a neutral state, with a buffered jump based on character movement and input buffer
    fn buffer_jump(motion: u8, movement: &Movement, superjump: bool) -> Self {
        let x_velocity = match motion {
            7 => movement.facing_vector * (-movement.back_walk_speed * 1.75),
            9 => movement.facing_vector * (movement.walk_speed),
            _ => 0.0,
        };

        let y_velocity = if superjump {
            movement.jump_height * 1.25
        } else {
            movement.jump_height
        };

        let velocity = Vec2::new(x_velocity, y_velocity);
        Self::Jumpsquat {
            duration: 3,
            velocity,
        }
    }

    /// If the new state does not match the old state, generate an animation transition
    fn calculate_transition(&self, other: &Self) -> Option<AnimationTransition> {
        use AnimationTransition::*;
        use CharacterState::*;
        match (self, other) {
            (Rising, Falling) => Some(RiseToFall),
            (Falling, Idle) | (Rising, Idle) => Some(FallToIdle),
            (Crouching, Idle) => Some(CrouchToIdle),
            (Walking, Idle) => Some(WalkToIdle),
            (BackWalking, Idle) => Some(BackwalkToIdle),
            (Dashing, Idle) => Some(DashToIdle),
            (BackDashing { duration: _ }, Idle) => Some(BackDashToIdle),
            (
                AirDashing {
                    duration: _,
                    velocity: _,
                },
                Falling,
            ) => Some(AirdashToFall),
            (
                AirBackDashing {
                    duration: _,
                    velocity: _,
                },
                Falling,
            ) => Some(AirbackdashToFall),
            (_, Idle) => Some(ToIdle),
            (
                _,
                Jumpsquat {
                    duration: _,
                    velocity: _,
                },
            ) => Some(ToRise),
            (_, Walking) => Some(ToWalk),
            (_, BackWalking) => Some(ToBackwalk),
            (_, Dashing) => Some(ToDash),
            (_, BackDashing { duration: _ }) => Some(ToBackdash),
            (
                _,
                AirDashing {
                    duration: _,
                    velocity: _,
                },
            ) => Some(ToAirdash),
            (
                _,
                AirBackDashing {
                    duration: _,
                    velocity: _,
                },
            ) => Some(ToAirBackdash),
            (_, Crouching) => Some(ToCrouch),
            (_, _) => None,
        }
    }

    /// Returns whether or not the character can turn around, based on current state
    pub fn get_can_turn(&self) -> bool {
        use CharacterState::*;
        matches!(
            self,
            Idle | Walking | BackWalking | Crouching | Rising | Falling
        )
    }

    /// Returns whether or not the character can block, based on their current state
    pub fn get_can_block(&self) -> bool {
        use CharacterState::*;
        match self {
            Idle | Walking | BackWalking | Crouching | Falling | Dashing => true,
            _ => false,
        }
    }

    /// Returns whether or not the character is in the air, based on current state
    pub fn get_airborne(&self) -> bool {
        use CharacterState::*;
        matches!(
            self,
            AirJumpsquat {
                duration: _,
                velocity: _
            } | Rising
                | Falling
                | AirDashing {
                    duration: _,
                    velocity: _
                }
                | AirBackDashing {
                    duration: _,
                    velocity: _
                }
        )
    }

    /// Updates the current state if finishing an airdash
    pub fn is_finished_airdashing(&self) -> bool {
        use CharacterState::*;
        match self {
            AirDashing {
                duration,
                velocity: _,
            } => *duration == 0,
            AirBackDashing {
                duration,
                velocity: _,
            } => *duration == 0,
            _ => false,
        }
    }

    /// Called when the character lands, forcing them into a Idle state
    pub fn land(&mut self) {
        use CharacterState::*;
        *self = Idle;
    }
}
