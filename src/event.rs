//! Define and register all events

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<JumpEvent>();
}

/// Event sent when the player jump.
#[derive(Event, Debug)]
pub struct JumpEvent;
