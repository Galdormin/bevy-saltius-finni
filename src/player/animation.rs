//! Animation of the player

use std::time::Duration;

use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    assets::collections::PlayerAssets,
    player::{
        movement::Dead,
        physics::{CharacterController, Grounded},
    },
    utils::animation::{AnimationState, update_animation_atlas, update_sprite_animation},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_animation_atlas::<PlayerAnimationState>,
            update_sprite_animation::<PlayerAnimationState>,
            update_animation_movement,
        )
            .run_if(resource_exists::<PlayerAssets>),
    );
}

/// The different state of the animation for the player
#[derive(Clone, Component, Reflect, PartialEq)]
pub enum PlayerAnimationState {
    Idle,
    Walk,
    Jump,
    Fall,
    Dead,
}

impl AnimationState for PlayerAnimationState {
    fn get_frames(&self) -> usize {
        match self {
            Self::Idle => 2,
            Self::Walk => 6,
            _ => 1,
        }
    }

    fn get_duration(&self) -> Option<Duration> {
        match self {
            Self::Idle => Some(Duration::from_millis(500)),
            Self::Walk => Some(Duration::from_millis(100)),
            _ => None,
        }
    }

    fn get_start_frame(&self) -> usize {
        match self {
            PlayerAnimationState::Idle => 0,
            PlayerAnimationState::Walk => 6,
            PlayerAnimationState::Jump => 12,
            PlayerAnimationState::Fall => 18,
            PlayerAnimationState::Dead => 24,
        }
    }
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
    mut player_query: Query<
        (
            &LinearVelocity,
            &mut Sprite,
            &mut PlayerAnimationState,
            Has<Grounded>,
            Has<Dead>,
        ),
        With<CharacterController>,
    >,
) {
    for (linear_velocity, mut sprite, mut animation_state, is_grounded, is_dead) in
        &mut player_query
    {
        if linear_velocity.x.abs() > 1.0 {
            sprite.flip_x = linear_velocity.x < 0.0;
        }

        let new_state = if is_dead {
            PlayerAnimationState::Dead
        } else if is_grounded {
            if linear_velocity.x.abs() < 1.0 {
                PlayerAnimationState::Idle
            } else {
                PlayerAnimationState::Walk
            }
        } else if linear_velocity.y > 0.0 {
            PlayerAnimationState::Jump
        } else {
            PlayerAnimationState::Fall
        };

        if new_state != *animation_state {
            *animation_state = new_state;
        };
    }
}
