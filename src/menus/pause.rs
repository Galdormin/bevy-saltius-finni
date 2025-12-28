//! The pause menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use bevy_cobweb_ui::prelude::*;

use crate::menus::Menu;

pub(super) fn plugin(app: &mut App) {
    app.load("ui/cobweb/pause.cob");

    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands, mut scene_builder: SceneBuilder) {
    commands.ui_root().spawn_scene(
        ("ui/cobweb/pause.cob", "scene"),
        &mut scene_builder,
        |handle| {
            handle.insert((DespawnOnExit(Menu::Pause), GlobalZIndex(2)));
        },
    );
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
