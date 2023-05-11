use crate::character::actions::Action;
use crate::types::{Frame, PlayerId};
use crate::{CommandInput, InputTree, PlayerInputSources, PlayerPositions};
use bevy::prelude::*;
use bevy::utils::HashMap;
use dyn_clone::clone_box;

pub struct PlayerActions(Vec<PlayerActionLibrary>);

impl PlayerActions {
    pub fn get_library_mut(&mut self, player_id: &PlayerId) -> &mut PlayerActionLibrary {
        self.0
            .iter_mut()
            .find(|x| x.get_player_id() == player_id)
            .unwrap()
    }

    pub fn get_library(&self, player_id: &PlayerId) -> &PlayerActionLibrary {
        self.0
            .iter()
            .find(|x| x.get_player_id() == player_id)
            .unwrap()
    }
}

impl Default for PlayerActions {
    fn default() -> Self {
        PlayerActions(vec![
            PlayerActionLibrary::new(PlayerId::P1),
            PlayerActionLibrary::new(PlayerId::P2),
        ])
    }
}

pub struct PlayerActionLibrary {
    pub player_id: PlayerId,
    action_library: ActionLibrary,
}

impl PlayerActionLibrary {
    pub fn new(player_id: PlayerId) -> Self {
        PlayerActionLibrary {
            player_id,
            action_library: ActionLibrary::default(),
        }
    }
    pub fn get_player_id(&self) -> &PlayerId {
        &self.player_id
    }

    pub fn find_action(&self, input_tree: &InputTree) -> Option<Box<dyn Action>> {
        self.action_library.find_action(input_tree)
    }
}

#[derive(Clone)]
pub struct ActionLibrary {
    pub character_id: String,
    actions: HashMap<CommandInput, Box<dyn Action>>,
}

impl ActionLibrary {
    pub fn new(character_id: String, actions: HashMap<CommandInput, Box<dyn Action>>) -> Self {
        ActionLibrary {
            character_id,
            actions,
        }
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

impl Default for ActionLibrary {
    fn default() -> Self {
        ActionLibrary {
            character_id: "Empty".to_string(),
            actions: HashMap::new(),
        }
    }
}

pub fn transition_character_states(
    player_buffers: Res<PlayerInputSources>,
    player_positions: Res<PlayerPositions>,
    mut player_query: Query<(&PlayerId, &mut CharacterState)>,
) {
    for (player_id, mut character_state) in &mut player_query {
        let buffer = player_buffers.get_source(player_id);
        let tree = buffer.build_input_tree(player_positions.get_facing_right(player_id));
    }
}

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
