//! The screen state for the main gameplay.

use avian2d::prelude::{Physics, PhysicsTime};
use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};

use crate::{
    Pause, event::DeathEvent, menus::Menu, platformer::level::spawn_level, screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);

    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
            open_death_menu.run_if(in_state(Screen::Gameplay).and(not(in_state(Menu::Death)))),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>, mut physic_time: ResMut<Time<Physics>>) {
    physic_time.unpause();
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>, mut physic_time: ResMut<Time<Physics>>) {
    physic_time.pause();
    next_pause.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(Pause(true)),
    ));
}

fn open_death_menu(
    mut commands: Commands,
    death_event: EventReader<DeathEvent>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    if death_event.is_empty() {
        return;
    }

    commands.spawn((
        Name::new("Death Overlay"),
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(Menu::Death),
    ));

    next_menu.set(Menu::Death);
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
