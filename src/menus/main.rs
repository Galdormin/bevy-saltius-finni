//! The main menu (seen on the title screen).

use bevy::prelude::*;

use bevy_cobweb_ui::prelude::*;

use crate::menus::Menu;

pub(super) fn plugin(app: &mut App) {
    app.load("ui/cobweb/main.cob")
        .add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands, mut scene_builder: SceneBuilder) {
    commands.ui_root().spawn_scene(
        ("ui/cobweb/main.cob", "scene"),
        &mut scene_builder,
        |handle| {
            handle.insert(StateScoped(Menu::Main));
        },
    );
}
