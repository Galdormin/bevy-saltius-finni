//! Load UI Assets

use bevy::prelude::*;

use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;

use sf_ui::prelude::Screen;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(Screen::Loading)
            .load_collection::<UiAssets>()
            .load_collection::<PlayerAssets>()
            .load_collection::<LevelAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "ui/button.png")]
    pub button_image: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 40, tile_size_y = 27, columns = 2, rows = 1))]
    pub button_atlas: Handle<TextureAtlasLayout>,

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
                border: BorderRect {
                    left: 5.0,
                    right: 5.0,
                    top: 5.0,
                    bottom: 6.0,
                },
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "sprites/character_simple.png")]
    pub sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 6, rows = 5))]
    pub atlas: Handle<TextureAtlasLayout>,
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(path = "world.ldtk")]
    pub world: Handle<LdtkProject>,
    #[asset(path = "audio/music/Going Up.ogg")]
    pub music: Handle<AudioSource>,
}
