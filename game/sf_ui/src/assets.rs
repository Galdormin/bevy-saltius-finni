//! UI assets loaded during the Loading screen.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::states::Screen;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(Screen::Loading).load_collection::<UiAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "ui/button.png")]
    pub button_image: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 40, tile_size_y = 27, columns = 2, rows = 1))]
    pub button_atlas: Handle<TextureAtlasLayout>,

    // Images
    #[asset(path = "icons/banner.png")]
    pub banner: Handle<Image>,
    #[asset(path = "ui/jump_icon.png")]
    pub jump_icon: Handle<Image>,

    // Fonts
    #[asset(path = "fonts/monogram.ttf")]
    pub monogram: Handle<Font>,
    #[asset(path = "fonts/m6x11.ttf")]
    pub m6x11: Handle<Font>,

    // Sounds
    #[asset(path = "audio/sound_effects/button_click.ogg")]
    pub hover_sound: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/button_click.ogg")]
    pub click_sound: Handle<AudioSource>,
}

impl UiAssets {
    pub fn button_image_node(&self) -> ImageNode {
        ImageNode {
            image: self.button_image.clone(),
            texture_atlas: Some(TextureAtlas::from(self.button_atlas.clone())),
            image_mode: NodeImageMode::Sliced(TextureSlicer {
                border: [5.0, 5.0, 5.0, 6.0].into(),
                ..default()
            }),
            ..default()
        }
    }
}
