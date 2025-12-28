//! The credits menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use bevy_cobweb_ui::prelude::*;

use crate::{assets::collections::LevelAssets, audio::music, menus::Menu};

pub(super) fn plugin(app: &mut App) {
    app.load("ui/cobweb/credits.cob");
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(OnEnter(Menu::Credits), start_credits_music);
}

fn spawn_credits_menu(mut commands: Commands, mut scene_builder: SceneBuilder) {
    commands.ui_root().spawn_scene(
        ("ui/cobweb/credits.cob", "scene"),
        &mut scene_builder,
        |handle| {
            handle.insert(DespawnOnExit(Menu::Credits));
        },
    );
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn start_credits_music(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Credits Music"),
        DespawnOnExit(Menu::Credits),
        music(level_assets.music.clone()),
    ));
}
