use bevy::prelude::*;

pub mod button;
pub mod checkpoint;
pub mod door;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((button::plugin, door::plugin, checkpoint::plugin));
}
