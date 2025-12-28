//! Game HUD that have some information (jump amount, etc)

use bevy::prelude::*;

use bevy_cobweb_ui::prelude::*;

use crate::{
    player::{movement::JumpAmount, physics::CharacterController},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.load("ui/cobweb/hud.cob");
    app.register_component_type::<JumpCounter>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_hud);

    app.add_systems(
        Update,
        update_jump_counter.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component, Reflect, Debug, Default, PartialEq)]
#[reflect(Component)]
pub struct JumpCounter;

fn spawn_hud(mut commands: Commands, mut scene_builder: SceneBuilder) {
    commands
        .ui_root()
        .spawn_scene(("ui/cobweb/hud.cob", "hud"), &mut scene_builder, |handle| {
            handle.insert((DespawnOnExit(Screen::Gameplay), GlobalZIndex(1)));
        });
}

fn update_jump_counter(
    jump_amount: Single<&JumpAmount, (With<CharacterController>, Changed<JumpAmount>)>,
    mut jump_counter: Single<&mut Text, With<JumpCounter>>,
) {
    jump_counter.0 = jump_amount.remaining.to_string();
}
