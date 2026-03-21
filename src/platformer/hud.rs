//! Game HUD that have some information (jump amount, etc)

use bevy::prelude::*;
use bevy::ui::Val::*;

use sf_ui::prelude::Screen;

use crate::assets::collections::UiAssets;
use crate::player::{movement::JumpAmount, physics::CharacterController};
use crate::ui::theme::UiTheme;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<JumpCounter>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_hud);
    app.add_systems(
        Update,
        update_jump_counter.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component, Reflect, Debug, Default, PartialEq)]
#[reflect(Component)]
pub struct JumpCounter;

fn spawn_hud(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((
        Name::new("HUD"),
        Node {
            position_type: PositionType::Absolute,
            left: Px(2.0),
            top: Px(2.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        GlobalZIndex(1),
        DespawnOnExit(Screen::Gameplay),
        children![
            (
                Name::new("Jump Icon"),
                ImageNode {
                    image: ui_assets.jump_icon.clone(),
                    ..default()
                },
                Node {
                    margin: UiRect::all(Px(1.0)),
                    ..default()
                },
            ),
            (
                Name::new("x"),
                Text("x".into()),
                UiTheme::PIXEL_ART,
                TextFont::from_font_size(13.0),
                Node {
                    margin: UiRect::all(Px(1.0)),
                    ..default()
                },
            ),
            (
                Name::new("Jump Counter"),
                Text("5".into()),
                UiTheme::PIXEL_ART,
                TextFont::from_font_size(13.0),
                Node {
                    margin: UiRect::all(Px(1.0)),
                    ..default()
                },
                JumpCounter,
            ),
        ],
    ));
}

fn update_jump_counter(
    jump_amount: Single<&JumpAmount, (With<CharacterController>, Changed<JumpAmount>)>,
    mut jump_counter: Single<&mut Text, With<JumpCounter>>,
) {
    jump_counter.0 = jump_amount.remaining.to_string();
}
