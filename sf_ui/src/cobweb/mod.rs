//! Module with all code related to CobWeb

use bevy::prelude::*;

mod attributes;
pub mod buttons;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((buttons::plugin, attributes::plugin));
}

pub(super) mod prelude {
    pub use super::buttons::{ButtonStates, CobButtonRegistration, CubButtonStateRegistration};
}
