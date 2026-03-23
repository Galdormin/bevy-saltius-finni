//! The game's menus and transitions between them.

mod credits;
mod main_menu;
mod pause;
mod settings;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        credits::plugin,
        main_menu::plugin,
        settings::plugin,
        pause::plugin,
    ));
}
