use bevy::prelude::*;

pub mod animation;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(animation::plugin);
}
