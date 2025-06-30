//! Simple platfomer to write the platformer physic engine

use bevy::prelude::*;

pub mod entities;
mod hud;
pub mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, hud::plugin, entities::plugin));
}
