pub mod plugin;
pub mod states;

pub mod prelude {
    pub use crate::plugin::SfUiPlugin;
    pub use crate::states::{Menu, Screen};
}
