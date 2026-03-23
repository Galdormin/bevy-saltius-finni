//! Load game assets

use bevy::prelude::*;

use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;

use sf_ui::prelude::Screen;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(Screen::Loading)
            .load_collection::<PlayerAssets>()
            .load_collection::<LevelAssets>(),
    );
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
