use bevy::prelude::*;

use crate::cobweb;
use crate::states;

pub struct SfUiPlugin;

impl Plugin for SfUiPlugin {
    fn build(&self, app: &mut App) {
        info!("Plugin Registered");

        app.add_plugins((cobweb::plugin, states::plugin));
    }
}
