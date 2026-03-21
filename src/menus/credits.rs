//! The credits menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};

use sf_ui::prelude::Menu;

use crate::{assets::collections::LevelAssets, audio::music, ui::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Menu::Credits),
        (spawn_credits_menu, start_credits_music),
    );
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_credits_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Credits Menu"),
        DespawnOnExit(Menu::Credits),
        children![
            widget::header("Created by"),
            (
                Name::new("Team Grid"),
                Node {
                    display: Display::Grid,
                    column_gap: Px(12.0),
                    grid_auto_flow: GridAutoFlow::Row,
                    grid_template_columns: RepeatedGridTrack::px(2, 300.0),
                    ..default()
                },
                children![
                    (
                        widget::label("Galdormin"),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    widget::label("Game Designer & Programmer"),
                    (
                        widget::label("Nexia"),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    widget::label("Artist"),
                ],
            ),
            widget::header("Assets"),
            (
                Name::new("Assets Grid"),
                Node {
                    display: Display::Grid,
                    column_gap: Px(12.0),
                    grid_auto_flow: GridAutoFlow::Row,
                    grid_template_columns: RepeatedGridTrack::px(2, 300.0),
                    ..default()
                },
                children![
                    (
                        widget::label("Button SFX"),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    widget::label("Jaszunio15"),
                    (
                        widget::label("Music"),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    widget::label("Going Up by Ansimuz"),
                ],
            ),
            (
                Name::new("Spacer"),
                Node {
                    height: Px(20.0),
                    ..default()
                }
            ),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn go_back_on_click(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
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
