//! Code for the player character and its power-ups

use bevy::prelude::*;

pub mod genes;
pub mod movement;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((movement::plugin, genes::plugin));
}
