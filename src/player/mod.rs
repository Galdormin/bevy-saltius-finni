//! Code for the player character and its power-ups

use bevy::prelude::*;

pub mod genes;
pub mod movement;
pub mod physics;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((movement::plugin, genes::plugin, physics::plugin));
}
