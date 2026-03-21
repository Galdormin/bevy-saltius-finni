use bevy::prelude::*;

use crate::states;

pub struct SfUiPlugin;

impl Plugin for SfUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(states::plugin);
    }
}
