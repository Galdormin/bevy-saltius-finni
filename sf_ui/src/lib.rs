pub mod cobweb;
pub mod plugin;
pub mod states;

pub mod prelude {
    pub use crate::cobweb::prelude::*;
    pub use crate::plugin::SfUiPlugin;
    pub use crate::states::{Menu, Screen};
}
