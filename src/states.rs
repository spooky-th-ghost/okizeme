use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::ecs::world;
use bevy::utils::HashMap;
use bevy::{ecs::query::WorldQuery, prelude::*};

use crate::{PlayerId, Velocity};

#[derive(Resource)]
pub struct CharacterActions {
    forward_ground_dash: GroundDashV2,
    backward_ground_dash: GroundDashV2,
    forward_air_dash: AirdashV2,
    backward_air_dash: AirdashV2,
    attacks: HashMap<String, Box<dyn AttackTrait>>,
}

fn blah(world: &mut World) {
    let mut attacks: HashMap<String, Box<dyn AttackTrait>> = HashMap::new();
    attacks.insert("5K".to_string(), Box::new(Shoto5K));
    attacks.insert("6S".to_string(), Box::new(Sol6S));
    let actions = CharacterActions {
        forward_ground_dash: GroundDashV2::Run { speed: Speed(0.5) },
        backward_ground_dash: GroundDashV2::Run { speed: Speed(1.0) },
        forward_air_dash: AirdashV2::Straight { speed: Speed(1.0) },
        backward_air_dash: AirdashV2::Straight { speed: Speed(2.0) },
        attacks,
    };

    let fiveK = Box::new(Shoto5K);
    fiveK.execute(Entity::from_raw(41), world);
    fiveK.execute(Entity::from_raw(41), world);
}

#[derive(Component)]
pub struct AttackHandler {
    attack: Box<dyn AttackTrait>,
}

pub struct Shoto5K;

impl AttackTrait for Shoto5K {
    fn execute(&self, entity: Entity, mut world: &mut World) {
        let mut player_query = world.query::<(&PlayerId, &mut Velocity)>();
        if let Ok((player_id, mut velocity)) = player_query.get_mut(&mut world, entity) {}
    }
}

pub struct Sol6S;

impl AttackTrait for Sol6S {
    fn execute(&self, entity: Entity, world: &mut World) {}
}

pub trait AttackTrait: Send + Sync + 'static {
    fn execute(&self, entity: Entity, world: &mut World);
}

pub enum CharacterAction {
    Dash(GroundDashV2),
    AirDash(AirdashV2),
    Attack(AttackV2),
}

#[derive(Component)]
pub enum CharacterStateV2 {
    Idle,
    Walking,
    Backwalking,
    AttackingGrounded {
        attack: AttackV2,
    },
    AttackingAirborne {
        attack: AttackV2,
    },
    Freefall {
        recovery: Recovery,
    },
    Crouching,
    Jumpsquat {
        duration: Duration,
        jump_velocity: Vec2,
    },
    Rising,
    Falling,
    Juggle,
    Dashing {
        dash_type: GroundDashV2,
    },
    BackDashing {
        dash_type: GroundDashV2,
    },
    AirDashing {
        airdash_type: AirdashV2,
    },
    AirBackDashing {
        airdash_type: AirdashV2,
    },
    Blocking,
    AirBlocking,
}

impl CharacterStateV2 {
    pub fn get_state_type(&self) -> StateType {
        use CharacterStateV2::*;
        match *self {
            Idle
            | Walking
            | Backwalking
            | Crouching
            | Blocking
            | AttackingGrounded { attack: _ }
            | Dashing { dash_type: _ }
            | BackDashing { dash_type: _ } => StateType::Grounded,
            _ => StateType::Airborne,
        }
    }
    pub fn transition_state(&mut self, commands: &mut Commands, entity: Entity) {
        use StateType::*;
        // Simple transition may not work here,
        // what I need is this:
        // 1. Raw input comes in from any source
        // 2. Input is parsed into a list of possible actions ranked by priority
        // 3. Highest priority that matches a possible character action get's executed
        // 4. that execution triggers a transition
        //
        // This could be an iteratable InputTree type
        // in a while loop we match against input_tree.next()
        // as soon as we find anything that triggers a valid transition we load that
        //
        // Input tree in it's raw form would contain each possible parsed and sorted combination
        // example:
        // if we find a motion, we search 5 frames before it for a button
        // for each motion we pass an input with that motion and each button that matches in button
        // priority order eg:
        // 236D > 236C > 236A
        // instead of the `until` pattern currently in the parser, each valid input needs to be
        // recorded, it seems more efficient in the long run to have the whole tree instead of
        // considering character possible inputs in the parsing step
        // for normals we need to interpret each possible motion
        // 4 and 6 generate an additional 5 input when paired with a button
        // 2 inputs take priority and follow the same logic, coercing 1 and 3 to also send a 2
        // forward > backward > neutral priority order
        // once we have the entire tree sorted in priority order we pass that
        // character actions.parse() method takes in a full input tree and returns an action for
        // the first valid input it finds
        // that is the correct action to take that frame
        //
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
#[derive(Clone, Copy)]
pub struct AttackV2 {
    counterhit: SimpleRange,
}

pub struct HurtboxV2 {
    lifetime: SimpleRange,
    position: Vec2,
}

pub enum GroundDashV2 {
    Run {
        speed: Speed,
    },
    StepDash {
        speed: Speed,
        duration: Duration,
        airborne: SimpleRange,
    },
    Teleport {
        distance: Distance,
        duration: Duration,
        invulnerability: SimpleRange,
    },
}

pub enum AirdashV2 {
    Straight { speed: Speed },
    Hover { direction: Vec2, speed: Speed },
}

#[derive(Debug, Clone, Copy)]
pub struct SimpleRange {
    start: u8,
    end: u8,
}

impl SimpleRange {
    pub fn start(&self) -> u8 {
        self.start
    }

    pub fn end(&self) -> u8 {
        self.end
    }

    pub fn contains(&self, value: u8) -> bool {
        value <= self.end || value >= self.start
    }
}

pub struct Duration(u8);
pub struct Recovery(u8);
pub struct Speed(f32);
pub struct Distance(f32);
