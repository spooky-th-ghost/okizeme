use crate::character::actions::{Airdash, Attack, Dash};
use crate::types::Frame;
use crate::{CommandInput, CommandMotion, InputTree, PlayerId};
use bevy::prelude::*;
use bevy::utils::HashMap;
use dyn_clone::clone_box;

#[derive(Resource)]
pub struct CharacterActions {
    forward_ground_dash: Box<dyn Dash>,
    backward_ground_dash: Box<dyn Dash>,
    forward_air_dash: Box<dyn Airdash>,
    backward_air_dash: Box<dyn Airdash>,
    attack_library: AttackLibrary,
}

impl CharacterActions {
    pub fn find_attack(&self, input_tree: &InputTree) -> Option<Box<dyn Attack>> {
        self.attack_library.find_attack(input_tree)
    }
}

pub struct AttackLibrary {
    attacks: HashMap<CommandInput, Box<dyn Attack>>,
}

impl AttackLibrary {
    pub fn find_attack(&self, input_tree: &InputTree) -> Option<Box<dyn Attack>> {
        let mut keys: Vec<&CommandInput> = self.attacks.keys().collect();
        keys.sort_by(|a, b| a.motion().partial_cmp(&b.motion()).unwrap());

        for key in keys {
            for input in input_tree.command_inputs() {
                if key.motion() == input.motion() {
                    let key_button = key.button();
                    if key_button & input.button() == key_button {
                        if let Some(attack) = self.attacks.get(&input) {
                            return Some(*clone_box(attack));
                        }
                    }
                }
            }
        }
        None
    }
}

pub fn transition_character_states() {}

pub fn execute_character_actions(mut commands: Commands, player_query: Query<&CharacterState>) {
    for character_state in &player_query {
        if let Some(attack) = character_state.is_attacking() {
            commands.add(attack);
        }
        if let Some(airdash) = character_state.is_airdashing() {
            commands.add(airdash);
        }
        if let Some(dash) = character_state.is_dashing() {
            commands.add(dash);
        }
    }
}

#[derive(Component)]
pub enum CharacterState {
    Idle,
    Walking,
    Backwalking,
    AttackingGrounded {
        frame: Frame,
        attack: Box<dyn Attack>,
    },
    AttackingAirborne {
        frame: Frame,
        attack: Box<dyn Attack>,
    },
    Freefall {
        recovery: Frame,
    },
    Crouching,
    Jumpsquat {
        frame: Frame,
        jump_velocity: Vec2,
    },
    Rising,
    Falling,
    Juggle,
    Dashing {
        frame: Frame,
        dash: Box<dyn Dash>,
    },
    BackDashing {
        frame: Frame,
        dash: Box<dyn Dash>,
    },
    AirDashing {
        frame: Frame,
        airdash: Box<dyn Airdash>,
    },
    AirBackDashing {
        frame: Frame,
        airdash: Box<dyn Airdash>,
    },
    Blocking,
    AirBlocking,
}

impl CharacterState {
    pub fn get_state_type(&self) -> StateType {
        use CharacterState::*;
        match *self {
            Idle
            | Walking
            | Backwalking
            | Crouching
            | Blocking
            | AttackingGrounded {
                frame: _,
                attack: _,
            }
            | Dashing { frame: _, dash: _ }
            | BackDashing { frame: _, dash: _ } => StateType::Grounded,
            _ => StateType::Airborne,
        }
    }

    pub fn is_attacking(&self) -> Option<Box<dyn Attack>> {
        match self {
            CharacterState::AttackingGrounded { frame: _, attack } => Some(*clone_box(attack)),
            CharacterState::AttackingAirborne { frame: _, attack } => Some(*clone_box(attack)),
            _ => None,
        }
    }

    pub fn is_airdashing(&self) -> Option<Box<dyn Airdash>> {
        match self {
            CharacterState::AirDashing { frame: _, airdash } => Some(*clone_box(airdash)),
            CharacterState::AirBackDashing { frame: _, airdash } => Some(*clone_box(airdash)),
            _ => None,
        }
    }

    pub fn is_dashing(&self) -> Option<Box<dyn Dash>> {
        match self {
            CharacterState::Dashing { frame: _, dash } => Some(*clone_box(dash)),
            CharacterState::BackDashing { frame: _, dash } => Some(*clone_box(dash)),
            _ => None,
        }
    }

    pub fn frame(&self) -> u8 {
        use CharacterState::*;
        match self {
            AttackingAirborne { frame, attack: _ } => frame.get(),
            AttackingGrounded { frame, attack: _ } => frame.get(),
            AirDashing { frame, airdash: _ } => frame.get(),
            AirBackDashing { frame, airdash: _ } => frame.get(),
            Dashing { frame, dash: _ } => frame.get(),
            BackDashing { frame, dash: _ } => frame.get(),
            Jumpsquat {
                frame,
                jump_velocity: _,
            } => frame.get(),
            _ => 0,
        }
    }

    pub fn tick(&mut self) {
        match self {
            CharacterState::AttackingGrounded { frame, attack: _ } => {
                frame.increment();
            }
            CharacterState::AttackingAirborne { frame, attack: _ } => {
                frame.increment();
            }
            CharacterState::Jumpsquat {
                frame,
                jump_velocity: _,
            } => {
                frame.increment();
            }
            _ => (),
        }
    }
}

pub enum StateType {
    Grounded,
    Airborne,
}

pub enum StateTransition {
    Attacking,
    Jumping,
    DashingForward,
    DashingBackward,
    Blocking,
    WalkingForward,
    WalkingBackward,
}

// When entering a state that has a duration (some dashes, all attacks)
// insert a Busy component
//
// after more thought it may be good to use different components for each
// [Attacking, Dashing, Jumping]
// in order to make some edge cases simpler and more elegant i.e:
// - Entities with [Attacking] are queried when looking to add a [Counterhit] component
// - Entites with [Dashing] will be easier to add custom animations for as well as indicate when a
// dash is airborne, if possible
//
// - Entites with [Jumping] are not ~really~ [Busy] they can cancel in to a special move and they
// cannot be thrown

// When an attack has a counter hit property
