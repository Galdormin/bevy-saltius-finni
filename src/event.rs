//! Define and register all events

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<JumpEvent>()
        .add_message::<DeathEvent>()
        .add_message::<RespawnEvent>();
}

/// Message sent when the player jumps.
#[derive(Message, Debug)]
pub struct JumpEvent;

/// Message sent when the player dies
#[derive(Message, Debug)]
pub struct DeathEvent;

/// Message sent to respawn the player
#[derive(Message, Debug)]
pub struct RespawnEvent;
