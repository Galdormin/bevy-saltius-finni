pub mod assets;
pub mod menus;
pub mod plugin;
pub mod states;
pub mod ui;

pub mod prelude {
    pub use crate::assets::UiAssets;
    pub use crate::plugin::SfUiPlugin;
    pub use crate::states::{Menu, Screen};
    pub use crate::ui::{
        interaction::InteractionPalette, palette as ui_palette, theme::UiTheme, widget,
    };
}
