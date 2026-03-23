//! The game's main screen states and transitions between them.

mod death_menu;
mod gameplay;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        death_menu::plugin,
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}
