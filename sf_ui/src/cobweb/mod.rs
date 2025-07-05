//! Module with all code related to CobWeb

use bevy::prelude::*;

pub mod buttons;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(buttons::plugin);
}

pub(super) mod prelude {
    pub use super::buttons::CobButtonRegistration;
}
