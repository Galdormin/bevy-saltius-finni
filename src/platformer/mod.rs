//! Simple platfomer to write the platformer physic engine

use bevy::prelude::*;

pub mod level;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin));
}
