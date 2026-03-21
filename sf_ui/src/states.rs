//! Define the UI states of the app

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>().init_state::<Menu>();
}

/// The different screens of the game
#[derive(Reflect, States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Gameplay,
}

/// The different menus of the app
#[derive(States, Copy, Clone, Eq, PartialEq, Reflect, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Pause,
    Death,
}
