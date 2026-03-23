use bevy::prelude::*;

use crate::{assets, menus, states, ui};

pub struct SfUiPlugin;

impl Plugin for SfUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((states::plugin, assets::plugin, ui::plugin, menus::plugin));
    }
}
