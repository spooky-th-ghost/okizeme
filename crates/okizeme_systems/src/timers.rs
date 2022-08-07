use bevy::prelude::*;
use okizeme_types::{
    AirdashLockout, Busy, Hitstop, JumpLockout, PlayerId, Stun, Timer, TimerEvent, TimerType,
};

pub fn manage_hitstop(mut coms: Commands, mut query: Query<(Entity, &mut Hitstop)>) {
    for (entity, mut hitstop) in query.iter_mut() {
        if hitstop.is_finished() {
            coms.entity(entity).remove::<Hitstop>();
        }
    }
}

pub fn manage_stun(mut coms: Commands, mut query: Query<(Entity, &mut Stun), Without<Hitstop>>) {
    for (entity, mut stun) in query.iter_mut() {
        if stun.is_finished() {
            coms.entity(entity).remove::<Stun>();
        }
    }
}

pub fn manage_busy(mut coms: Commands, mut query: Query<(Entity, &mut Busy), Without<Hitstop>>) {
    for (entity, mut busy) in query.iter_mut() {
        if busy.is_finished() {
            coms.entity(entity).remove::<Busy>();
        }
    }
}

pub fn manage_airdash_lockout(
    mut coms: Commands,
    mut query: Query<(Entity, &mut AirdashLockout), Without<Hitstop>>,
) {
    for (entity, mut airdash_lockout) in query.iter_mut() {
        if airdash_lockout.is_finished() {
            coms.entity(entity).remove::<AirdashLockout>();
        }
    }
}

pub fn manage_jump_lockout(
    mut coms: Commands,
    mut query: Query<(Entity, &mut JumpLockout), Without<Hitstop>>,
) {
    for (entity, mut jump_lockout) in query.iter_mut() {
        if jump_lockout.is_finished() {
            coms.entity(entity).remove::<JumpLockout>();
        }
    }
}

pub fn add_timers(
    mut commands: Commands,
    query: Query<(Entity, &PlayerId)>,
    mut timer_reader: EventReader<TimerEvent>,
) {
    for event in timer_reader.iter() {
        for (entity, player_id) in query.iter() {
            if event.player_id == *player_id {
                match event.timer_type {
                    TimerType::Hitstop => {
                        commands.entity(entity).insert(Hitstop(event.duration));
                    }
                    TimerType::Stun => {
                        commands.entity(entity).insert(Stun(event.duration));
                    }
                    TimerType::Busy => {
                        commands.entity(entity).insert(Busy(event.duration));
                    }
                    TimerType::JumpLockout => {
                        commands.entity(entity).insert(JumpLockout(event.duration));
                    }
                    TimerType::AirdashLockout => {
                        commands
                            .entity(entity)
                            .insert(AirdashLockout(event.duration));
                    }
                }
            }
        }
    }
}
