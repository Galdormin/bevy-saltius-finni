//! Define and register all game events

use bevy::prelude::*;

pub struct SfEventsPlugin;

impl Plugin for SfEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<JumpEvent>()
            .add_message::<DeathEvent>()
            .add_message::<RespawnEvent>();
    }
}

/// Event sent when the player jumps.
#[derive(Message, Debug)]
pub struct JumpEvent;

/// Event sent when the player dies
#[derive(Message, Debug)]
pub struct DeathEvent;

/// Event sent when the player Respawn
#[derive(Message, Debug)]
pub struct RespawnEvent;
