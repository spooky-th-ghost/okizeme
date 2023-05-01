use crate::{Hitbox, HitboxBundle, PlayerId};
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct CharacterActions {
    forward_ground_dash: GroundDashV2,
    backward_ground_dash: GroundDashV2,
    forward_air_dash: AirdashV2,
    backward_air_dash: AirdashV2,
    attacks: HashMap<String, Box<dyn Attack>>,
}

impl CharacterActions {
    pub fn get_action_from_input(&self) -> CharacterActionType {
        CharacterActionType::AirDash(AirdashV2::Straight { speed: Speed(0.5) })
    }
}

pub trait Attack: Send + Sync + 'static {
    fn execute(&self, frame: u8, player_id: PlayerId, entity: Entity, world: &mut World);
    fn startup(&self) -> u8;
    fn active(&self) -> Vec<u8>;
    fn recovery(&self) -> u8;
}

fn execute_actions(world: &mut World) {
    let mut player_query = world.query::<(Entity, &PlayerId, &CharacterStateV2)>();
    for (entity, player_id, character_state) in player_query.iter(world) {
        if let Some((frame, attack)) = character_state.is_attacking() {
            attack.execute(frame.get(), *player_id, entity, world);
        }
    }
}

#[derive(Component)]
pub struct Attacking;

pub struct SingleHitbox {
    hitbox_event: HitboxEvent,
    hurtbox_events: Vec<HurtboxEvent>,
    total_duration: Frame,
    counter_hit_duration: Frame,
}

impl Attack for SingleHitbox {
    fn execute(&self, frame: u8, player_id: PlayerId, entity: Entity, world: &mut World) {
        let mut player = world.entity_mut(entity);

        if !player.contains::<Attacking>() {
            player.insert(Attacking);
        }

        if frame == self.hitbox_event.frame {
            player.with_children(|parent| {
                parent.spawn(HitboxBundle::new(
                    player_id,
                    self.hitbox_event.hitbox,
                    self.hitbox_event.position,
                    self.hitbox_event.size,
                ));
            });
        }
    }

    fn startup(&self) -> u8 {
        self.hitbox_event.frame
    }

    fn active(&self) -> Vec<u8> {
        vec![self.hitbox_event.hitbox.duration.0]
    }

    fn recovery(&self) -> u8 {
        let active_sum: u8 = self.active().iter().sum();
        self.total_duration.get() - self.startup() - active_sum
    }
}

pub struct SingleProjectile {
    startup: u8,
    position: Vec2,
    velocity: Vec2,
}

/// Build some structs that impl Attack Trait as a starter
/// write structs for Airdash and Dash and see if it can be done without world access or handle all
/// of it within a specific execute_actions system

pub struct HitboxEvent {
    frame: u8,
    position: Vec2,
    size: Vec2,
    hitbox: Hitbox,
}

pub struct HurtboxEvent {
    frame: u8,
    head: Option<HurtboxProperties>,
    torso: Option<HurtboxProperties>,
    arms: Option<HurtboxProperties>,
    legs: Option<HurtboxProperties>,
}

pub struct HurtboxProperties {
    position: Vec2,
    size: Vec2,
    invulnerability: bool,
}

pub enum CharacterActionType {
    Dash(GroundDashV2),
    AirDash(AirdashV2),
    Attack(Box<dyn Attack>),
}

#[derive(Component)]
pub enum CharacterStateV2 {
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
        duration: Frame,
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
            | AttackingGrounded {
                frame: _,
                attack: _,
            }
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

    pub fn is_attacking(&self) -> Option<(&Frame, &Box<dyn Attack>)> {
        match self {
            CharacterStateV2::AttackingGrounded { frame, attack } => {
                //attack.execute(frame.get(), *player_id, entity, world);
                Some((frame, attack))
            }
            CharacterStateV2::AttackingAirborne { frame, attack } => {
                //attack.execute(frame.get(), *player_id, entity, world);
                Some((frame, attack))
            }
            _ => None,
        }
    }

    pub fn tick(&mut self) {
        match self {
            CharacterStateV2::AttackingGrounded { frame, attack: _ } => {
                frame.increment();
            }
            CharacterStateV2::AttackingAirborne { frame, attack: _ } => {
                frame.increment();
            }
            CharacterStateV2::Jumpsquat {
                duration,
                jump_velocity: _,
            } => {
                duration.increment();
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
        duration: Frame,
        airborne: SimpleRange,
    },
    Teleport {
        distance: Distance,
        duration: Frame,
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

#[derive(Default)]
pub struct Frame(u8);
impl Frame {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn set(&mut self, value: u8) {
        self.0 = value;
    }
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}
#[derive(Default)]
pub struct Speed(f32);
impl Speed {
    pub fn get(&self) -> f32 {
        self.0
    }
    pub fn set(&mut self, value: f32) {
        self.0 = value;
    }
}
#[derive(Default)]
pub struct Distance(f32);
impl Distance {
    pub fn get(&self) -> f32 {
        self.0
    }
    pub fn set(&mut self, value: f32) {
        self.0 = value;
    }
}
