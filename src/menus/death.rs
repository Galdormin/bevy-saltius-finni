//! Death menu to choose the genes

use bevy::prelude::*;

use crate::{event::RespawnEvent, menus::Menu, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Death), spawn_death_menu);
}

fn spawn_death_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Death Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Death),
        children![
            widget::header("You have died!"),
            widget::button("Respawn", respawn),
        ],
    ));
}

fn respawn(
    _: Trigger<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut respawn_event: EventWriter<RespawnEvent>,
) {
    next_menu.set(Menu::None);
    respawn_event.write(RespawnEvent);
}
