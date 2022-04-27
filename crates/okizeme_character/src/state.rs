// use bevy::prelude::*;
// use okizeme_utils::*;
// use okizeme_input::{
//   Buffer
// };

// /// Handles the current state of a character
// #[cfg_attr(feature = "debug", derive(Inspectable))]
// #[derive(Debug, Clone, Component)]
// pub enum CharacterState {
//   Idle,
//   Walking,
//   BackWalking,
//   Attacking {
//     ///The number of frames until the action completes naturally
//     duration: u8,
//     ///The current attack being executed 
//     attack: Attack,
//     ///Can the current attack me cancelled prematurely 
//     cancellable: bool
//   },
//   AttackingAirborne {
//     ///The number of frames until the action completes naturally
//     duration: u8, 
//     ///The current attack being executed 
//     attack: Attack,
//     ///Can the current attack me cancelled prematurely 
//     cancellable: bool
//   },
//   Crouching,
//   Jumpsquat {
//     ///The number of frames until the action completes naturally
//     duration: u8,
//     ///The Velocity of the buffered jump 
//     velocity: Vec2 
//   },
//   AirJumpsquat {
//     ///The number of frames until the action completes naturally
//     duration: u8,
//     ///The velocity of the buffered jump  
//     velocity: Vec2 
//   },
//   Rising {
//     ///The number of frames until the player can act out of the state
//     busy: u8
//   },
//   Falling,
//   Juggle,
//   Dashing,
//   BackDashing {
//     ///The number of frames until the action completes naturally
//     duration: u8
//   },
//   AirDashing {
//     ///The number of frames until the player can act out of the state
//     busy: u8,
//     ///The number of frames until the action completes naturally
//     duration: u8,
//     ///The velocity of the air dash 
//     velocity: Vec2
//   },
//   AirBackDashing {
//     ///The number of frames until the player can act out of the state
//     busy: u8,
//     ///The number of frames until the action completes naturally 
//     duration: u8,
//     ///The velocity of the air dash
//     velocity: Vec2
//   } 
// }

// impl PartialEq for CharacterState {
//   fn eq(&self, other: &Self) -> bool {
//     std::mem::discriminant(self) == std::mem::discriminant(other)
//   }
// }

// impl Default for CharacterState {
//   fn default() -> Self {
//     CharacterState::Idle
//   }
// }

// impl CharacterState {
//   fn tick(&mut self) {
//     use CharacterState::*;
//     match self {
//       Attacking {duration, attack: _, cancellable:_} => { *duration = countdown(*duration);},
//       Jumpsquat {duration, velocity:_ } => { *duration = countdown(*duration);},
//       AirJumpsquat {duration, velocity: _ } => { *duration = countdown(*duration);},
//       BackDashing {duration} => { *duration = countdown(*duration);},
//       Rising {busy} => {*busy = countdown(*busy)},
//       AirDashing {busy,duration, velocity:_} => {
//         *busy = countdown(*busy); 
//         *duration = countdown(*duration);
//       },
//       AirBackDashing {busy, duration, velocity:_} => {
//         *busy = countdown(*busy); 
//          *duration = countdown(*duration);
//         },
//       _ => () 
//     }
//   } 
//   /// updates a character state, advancing it's timers and changing it based on input and character movement
//   pub fn update(&mut self, buffer: &mut Buffer, movement: &mut CharacterMovement, attacks: &mut CharacterAttacks, name: &Name, library: &CharacterLibrary, position: Vec3) -> Option<AnimationTransition> {
//     use CharacterState::*;
//     self.tick();
    
//     let new_state = match self {
//       Idle | Walking | BackWalking | Crouching => self.from_neutral_states(buffer, movement, attacks, name, library),
//       Dashing => self.from_dashing(buffer, movement),
//       Jumpsquat { duration:_,velocity:_ } => self.from_jump_squat(movement),
//       Rising { busy: _ } | Falling => self.from_neutral_airborne(buffer, movement, attacks, name, library, position),
//       BackDashing { duration:_ } => self.from_backdashing(buffer, movement, attacks, name, library,),
//       Attacking {duration:_, attack:_, cancellable:_} => self.from_attacking(buffer, movement, attacks, name, library),
//       AirDashing { busy:_,duration:_,velocity:_} | AirBackDashing { busy:_,duration:_,velocity:_} => self.from_air_dashing(buffer, movement, attacks, name, library),
//       _ => self.clone()
//     };
//     let transition = if self.clone() != new_state {
//       self.calculate_transition(&new_state)
//     } else {
//       None
//     };
//     *self = new_state;
//     return transition;
//   }

//   /// Returns a new state based on input from the following states:
//   ///  - Idle
//   ///  - Walking
//   ///  - Backwalking
//   ///  - Crouching
//   pub fn from_neutral_states(&self, buffer: &Buffer, movement: &mut CharacterMovement, attacks: &mut CharacterAttacks, name: &Name, library: &CharacterLibrary)  -> Self {
//     use CharacterState::*;
//     if let Some(attack) = attacks.attack_to_execute(buffer, name, library, true) {
//       return self.buffer_attack(attack);
//     }

//     if let Some(ct) = buffer.command_type {
//       match ct {
//         CommandType::DASH => return Dashing,
//         CommandType::BACK_DASH => return self.buffer_backdash(movement),
//       _ => ()
//       }               
//     }

//     match buffer.current_motion {
//       4 => return BackWalking,
//       6 => return Walking,
//       1 | 2 | 3 => return Crouching,
//       7 | 8 | 9 => return Self::buffer_jump(buffer.current_motion, &movement.clone(), false),
//       _ => return Idle
//     }
//   }

//   /// Returns a new state based on the current state when in jump squat
//   pub fn from_jump_squat(&self, movement: &mut CharacterMovement) -> Self{
//     use CharacterState::*;
//     match self {
//       Jumpsquat { duration, velocity } => {
//         if *duration == 0 {
//           movement.velocity = *velocity;
//           return Rising {busy: movement.jump_lockout};
//         } else {
//           return self.clone();
//         }
//       },
//       _ => return self.clone(),
//     };
//   }

//   /// Returns a new state based on input from dashing
//   pub fn from_dashing(&self, buffer: &Buffer, movement: &CharacterMovement) -> Self {
//     use CharacterState::*;
//     match buffer.current_motion {
//       4 => return BackWalking,
//       6 => return Dashing,
//       1 | 2 | 3 => return Crouching,
//       7 | 8 | 9 => return Self::buffer_dash_jump(buffer.current_motion, movement, false),
//       _ => return Idle
//     }
//   }

//   pub fn from_air_dashing(&self, buffer: &Buffer, movement: &mut CharacterMovement, attacks: &mut CharacterAttacks, name: &Name, library: &CharacterLibrary) -> Self {
//     use CharacterState::*;
//     match self {
//       AirDashing {busy:_ ,duration, velocity:_} => {
//         if *duration == 0 {
//           return self.from_neutral_airborne(buffer, movement, attacks, name, library, Vec3::ONE);
//         }
//         return self.clone();
//       },
//       AirBackDashing {busy:_,duration, velocity:_} => {
//         if *duration == 0 {
//           return self.from_neutral_airborne(buffer, movement, attacks, name, library, Vec3::ONE);
//         }
//         return self.clone();
//       },
//       _ => return self.clone(),
//     }
//   }

//   /// Returns a new state based on input from the following states:
//   ///  - Rising
//   ///  - Falling
//   ///  - Airdashing
//   ///  - Airbackdashing
//   pub fn from_neutral_airborne(&self, buffer: &Buffer, movement: &mut CharacterMovement, attacks: &mut CharacterAttacks, name: &Name, library: &CharacterLibrary, position: Vec3) -> Self {
//     use CharacterState::*;
//     if position.y <= 0.0 {
//       return Idle;
//     }
//     match self {
//       Rising { busy } => {
//         if *busy == 0 {
//           return self.from_airborne_input(buffer, movement, attacks, name, library);
//         } else {
//           return self.clone();
//         }
//       },
//       Falling | AirDashing {busy:_,duration:_,velocity:_} |  AirBackDashing {busy:_,duration:_,velocity:_} => {
//         return self.from_airborne_input(buffer, movement, attacks, name, library);
//       },
//       _ => return self.clone(),
//     };
//   }

//   /// Returns a new state based on input and the backdash timer from backdash
//   pub fn from_backdashing(&self, buffer: &Buffer, movement: &mut CharacterMovement, attacks: &mut CharacterAttacks, name: &Name, library: &CharacterLibrary) -> Self {
//     use CharacterState::*;
//     match self {
//       BackDashing {duration} => {
//         if *duration == 0 {
//           return self.from_neutral_states(buffer, movement, attacks, name, library);
//         }
//         return self.clone();
//       },
//       _ => return self.clone(),
//     }
//   }

//   /// Returns a new state based on input and the attack timer from attack
//   pub fn from_attacking(&self, buffer: &Buffer, movement: &mut CharacterMovement, attacks: &mut CharacterAttacks, name: &Name, library: &CharacterLibrary) -> Self {
//     use CharacterState::*;
//     match self {
//       Attacking {duration, attack:_, cancellable} => {
//         if *duration == 0 || *cancellable {
//           return self.from_neutral_states(buffer, movement, attacks, name, library);
//         }
//         return self.clone();
//       },
//       _ => return self.clone(),
//     }
//   }

//   // Returns a new state from input while aireborne
//   pub fn from_airborne_input(&self, buffer: &Buffer, movement: &mut CharacterMovement, attacks: &mut CharacterAttacks, name: &Name, library: &CharacterLibrary) -> Self {
//     use CharacterState::*;
//     if let Some(attack) = attacks.attack_to_execute(buffer, name, library, true) {
//       return self.buffer_attack(attack);
//     }

//     if movement.can_airdash() {
//       if let Some(ct) = buffer.command_type {
//         match ct {
//           CommandType::DASH => {
//             movement.spend_airdash();
//             return self.buffer_airdash(movement, true)
//           },
//           CommandType::BACK_DASH => {
//             movement.spend_airdash();
//             return self.buffer_airdash(movement, false)
//           },
//         _ => ()
//         }               
//       }
//     }

//     return match self {
//       AirDashing {busy:_, duration:_,velocity:_} => {
//         if self.is_finished_airdashing() {
//           Falling
//         } else {
//           self.clone()
//         }
//       },
//       AirBackDashing {busy:_, duration:_,velocity:_} => {
//         if self.is_finished_airdashing() {
//           Falling
//         } else {
//           self.clone()
//         }
//       },
//       Rising {busy:_} => {
//         if movement.is_falling() {
//           Falling
//         } else {
//           self.clone()
//         }
//       }
//       _ => self.clone()
//     }
//   }

//   /// Returns an attacking state, with the passed attack
//   fn buffer_attack(&self, attack: Attack) -> Self {
//     return CharacterState::Attacking {duration: attack.busy, attack: attack.clone(), cancellable: false}
//   }

//   /// Returns a backdashing state, based on movement
//   fn buffer_backdash(&self, movement: &mut CharacterMovement) -> Self {
//     use Backdash::*;
//     match movement.backdash {
//       Standard {speed, busy, motion_duration} => {
//         let i_force = InterpolatedForce::new(
//           Vec2::new(-speed * movement.facing_vector, 0.0),
//           Vec2::new(-2.0 * movement.facing_vector, 0.0),
//           motion_duration
//         );
//         movement.set_interpolated_force(i_force);
//         return CharacterState::BackDashing {duration: busy}
//       },
//       _ => return Self::Idle
//     }
//   }

//   fn buffer_airdash(&self, movement: &mut CharacterMovement, forward: bool) -> Self {
//     use CharacterState::*;
//     if forward {
//         return AirDashing {busy: 10, duration: movement.max_airdash_time, velocity: Vec2::X * movement.air_dash_speed * movement.facing_vector};
//       } else {
//         return AirBackDashing {busy: 10, duration: movement.max_air_backdash_time, velocity: Vec2::X * movement.air_dash_speed * -movement.facing_vector };
//       }
//   }

//   /// Returns a Jumpsquat state from a Dash state, with a buffered jump based on character movement and input buffer
//   fn buffer_dash_jump(motion: u8, movement: &CharacterMovement, superjump: bool) -> Self {
//     let x_velocity = match motion {
//       7 => movement.facing_vector * (-movement.back_walk_speed),
//       9 => movement.facing_vector * (movement.walk_speed * 2.0),
//       _ => movement.facing_vector * (movement.walk_speed * 0.5)
//     };

//     let y_velocity = if superjump {
//       movement.jump_height * 1.25
//     } else {
//       movement.jump_height
//     };
    
//     let velocity = Vec2::new(x_velocity, y_velocity);
//     return Self::Jumpsquat {duration: 3, velocity}
//   }

//   /// Returns a Jumpsquat state from a neutral state, with a buffered jump based on character movement and input buffer
//   fn buffer_jump(motion:u8, movement: &CharacterMovement, superjump: bool) -> Self {
//     let x_velocity = match motion {
//       7 => movement.facing_vector * (-movement.back_walk_speed*1.75),
//       9 => movement.facing_vector * (movement.walk_speed),
//       _ => 0.0
//     };

//     let y_velocity = if superjump {
//       movement.jump_height * 1.25
//     } else {
//       movement.jump_height
//     };
    
//     let velocity = Vec2::new(x_velocity, y_velocity);
//     return Self::Jumpsquat {duration: 3, velocity}
//   }

//   /// If the new state does not match the old state, generate an animation transition
//   fn calculate_transition(&self, other: &Self) -> Option<AnimationTransition> {
//     use CharacterState::*;
//     use AnimationTransition::*;
//     match (self, other) {
//       (Rising {busy:_}, Falling) => Some(RiseToFall),
//       (Falling, Idle) | (Rising {busy:_}, Idle) => Some(FallToIdle),
//       (Crouching,Idle) => Some(CrouchToIdle),
//       (Walking,Idle) => Some(WalkToIdle),
//       (BackWalking,Idle) => Some(BackwalkToIdle),
//       (Dashing,Idle) => Some(DashToIdle),
//       (BackDashing { duration:_},Idle) => Some(BackDashToIdle),
//       (AirDashing {busy:_, duration:_, velocity:_}, Falling) => Some(AirdashToFall),
//       (AirBackDashing {busy:_, duration:_, velocity:_}, Falling) => Some(AirbackdashToFall),
//       (_, Idle) => Some(ToIdle),
//       (_, Jumpsquat {duration:_, velocity:_}) => Some(ToRise),
//       (_, Walking) => Some(ToWalk),
//       (_, BackWalking) => Some(ToBackwalk),
//       (_, Dashing) => Some(ToDash),
//       (_, BackDashing {duration:_}) => Some(ToBackdash),
//       (_, AirDashing {busy:_, duration:_, velocity:_}) => Some(ToAirdash),
//       (_, AirBackDashing {busy:_, duration:_, velocity:_}) => Some(ToAirBackdash),
//       (_, Crouching) => Some(ToCrouch),
//       (_, Attacking {duration:_, attack, cancellable:_}) => Some(ToAttack {name: attack.name.clone()}),
//       (_,_) => None
//     }
//   }

//   pub fn get_hitbox_events_this_frame(&self) -> Option<Vec<HitboxEvent>> {
//     use CharacterState::*;
//     if let Attacking{duration, attack, cancellable: _} = self.clone() {
//       let mut events = Vec::new();
//       for e in attack.hitbox_events.iter() {
//         if (attack.busy as i8 - e.frame as i8) == duration as i8 {
//           events.push(e.clone());
//         }
//       }
//       if events.len() != 0 {
//         return Some(events);
//       } else {
//         return None;
//       }
//     } else {
//       return None;
//     }
//   }

//   /// Returns whether or not the character can turn around, based on current state
//   pub fn get_can_turn(&self) -> bool {
//     use CharacterState::*;
//     match self {
//       Idle
//       | Walking
//       | BackWalking
//       | Crouching
//       | Rising {busy:_}
//       | Falling => return true,
//       _ => return false
//     }
//   }

//   /// Returns whether or not the character is in the air, based on current state
//   pub fn get_airborne(&self) -> bool {
//     use CharacterState::*;
//     match self {
//       AirJumpsquat {duration:_, velocity:_}
//       | Rising {busy:_}
//       | Falling
//       | AirDashing {busy:_, duration:_, velocity:_}
//       | AirBackDashing {busy:_, duration:_, velocity:_} => return true,
//       _ => return false
//     }
//   }

//   pub fn is_finished_airdashing(&self) -> bool {
//     use CharacterState::*;
//     match self {
//       AirDashing {busy:_, duration,velocity:_} => return *duration == 0,
//       AirBackDashing {busy:_, duration,velocity:_} => return *duration == 0,
//       _ => return false,
//     }
//   }

//   pub fn can_act_out_of_airdash(&self) -> bool {
//     use CharacterState::*;
//     match self {
//       AirDashing {busy, duration:_,velocity:_} => return *busy == 0,
//       AirBackDashing {busy, duration:_,velocity:_} => return *busy == 0,
//       _ => return false,
//     }
//   }

//   /// Called when the character lands, forcing them into a Idle state
//   pub fn land(&mut self) {
//     use CharacterState::*;
//     *self = Idle;
//   }
// }
