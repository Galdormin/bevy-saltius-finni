//! Define and register all events

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<JumpEvent>()
        .add_event::<DeathEvent>()
        .add_event::<RespawnEvent>();
}

/// Event sent when the player jumps.
#[derive(Event, Debug)]
pub struct JumpEvent;

/// Event sent when the player dies
#[derive(Event, Debug)]
pub struct DeathEvent;

/// Event sent to respawn the player
#[derive(Event, Debug)]
pub struct RespawnEvent;
