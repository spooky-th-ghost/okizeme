use crate::character::actions::Action;
use crate::types::Frame;
use crate::{CommandInput, InputTree};
use bevy::prelude::*;
use bevy::utils::HashMap;
use dyn_clone::clone_box;

#[derive(Resource)]
pub struct CharacterActions {
    action_library: ActionLibrary,
}

impl CharacterActions {
    pub fn find_action(&self, input_tree: &InputTree) -> Option<Box<dyn Action>> {
        self.action_library.find_action(input_tree)
    }
}

pub struct ActionLibrary {
    actions: HashMap<CommandInput, Box<dyn Action>>,
}

impl ActionLibrary {
    pub fn new(actions: HashMap<CommandInput, Box<dyn Action>>) -> Self {
        ActionLibrary { actions }
    }

    pub fn find_action(&self, input_tree: &InputTree) -> Option<Box<dyn Action>> {
        let mut keys: Vec<&CommandInput> = self.actions.keys().collect();
        keys.sort_by(|a, b| a.motion().partial_cmp(&b.motion()).unwrap());

        for key in keys {
            for input in input_tree.command_inputs() {
                if key.motion() == input.motion() {
                    let key_button = key.button();
                    if key_button & input.button() == key_button {
                        if let Some(actions) = self.actions.get(&input) {
                            return Some(*clone_box(actions));
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
        attack: Box<dyn Action>,
    },
    AttackingAirborne {
        frame: Frame,
        attack: Box<dyn Action>,
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
        dash: Box<dyn Action>,
    },
    BackDashing {
        frame: Frame,
        dash: Box<dyn Action>,
    },
    AirDashing {
        frame: Frame,
        airdash: Box<dyn Action>,
    },
    AirBackDashing {
        frame: Frame,
        airdash: Box<dyn Action>,
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

    pub fn is_attacking(&self) -> Option<Box<dyn Action>> {
        match self {
            CharacterState::AttackingGrounded { frame: _, attack } => Some(*clone_box(attack)),
            CharacterState::AttackingAirborne { frame: _, attack } => Some(*clone_box(attack)),
            _ => None,
        }
    }

    pub fn is_airdashing(&self) -> Option<Box<dyn Action>> {
        match self {
            CharacterState::AirDashing { frame: _, airdash } => Some(*clone_box(airdash)),
            CharacterState::AirBackDashing { frame: _, airdash } => Some(*clone_box(airdash)),
            _ => None,
        }
    }

    pub fn is_dashing(&self) -> Option<Box<dyn Action>> {
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
