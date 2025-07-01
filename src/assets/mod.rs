use bevy::prelude::*;

use bevy_asset_loader::prelude::*;

use crate::screens::Screen;

pub mod collections;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(Screen::Loading).continue_to_state(Screen::Title));

    app.add_plugins(collections::plugin);
}
