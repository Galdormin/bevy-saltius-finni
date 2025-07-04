//! Code for the player character and its power-ups

use bevy::prelude::*;

pub mod animation;
pub mod death;
pub mod genes;
pub mod movement;
pub mod physics;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        movement::plugin,
        genes::plugin,
        death::plugin,
        physics::plugin,
        animation::plugin,
    ));
}
