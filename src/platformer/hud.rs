//! Game HUD that have some information (jump amount, etc)

use bevy::prelude::*;

use crate::{
    player::{movement::JumpAmount, physics::CharacterController},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_jump_counter.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct JumpCounter;

fn update_jump_counter(
    jump_amount: Single<&JumpAmount, (With<CharacterController>, Changed<JumpAmount>)>,
    mut jump_counter: Single<&mut Text, With<JumpCounter>>,
) {
    jump_counter.0 = jump_amount.remaining.to_string();
}
