//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{
    assets::UiAssets,
    states::{Menu, Screen},
    ui::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        DespawnOnExit(Menu::Main),
        children![
            (
                Name::new("Banner"),
                ImageNode {
                    image: ui_assets.banner.clone(),
                    ..default()
                },
            ),
            widget::button("Play", go_to_gameplay),
            widget::button("Settings", go_to_settings),
            widget::button("Credits", go_to_credits),
            widget::button("Quit", quit_app),
        ],
    ));
}

fn go_to_gameplay(
    _: On<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(Menu::None);
    next_screen.set(Screen::Gameplay);
}

fn go_to_settings(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn go_to_credits(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

fn quit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
