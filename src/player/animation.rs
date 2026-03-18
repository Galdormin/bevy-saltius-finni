//! Animation of the player

use std::time::Duration;

use avian2d::prelude::LinearVelocity;
use bevy::{prelude::*, sprite::Anchor};

use crate::{
    assets::collections::PlayerAssets,
    player::{
        death::Dead,
        physics::{CharacterController, Grounded},
    },
    utils::animation::{
        AnimationState, SpriteAnimation, update_animation_atlas, update_sprite_animation,
    },
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

#[derive(Bundle, Default)]
pub struct CharacterSpriteBundle {
    sprite: Sprite,
    anchor: Anchor,
    sprite_animation: SpriteAnimation,
    player_animation_state: PlayerAnimationState,
}

impl CharacterSpriteBundle {
    pub fn from_player_assets(player_assets: Res<PlayerAssets>) -> Self {
        CharacterSpriteBundle {
            sprite: Sprite {
                image: player_assets.sprite.clone(),
                texture_atlas: Some(TextureAtlas::from(player_assets.atlas.clone())),
                ..default()
            },
            anchor: Anchor::from(Vec2::new(0.0, -0.2)),
            ..default()
        }
    }

    pub fn with_state(mut self, state: PlayerAnimationState) -> Self {
        self.sprite_animation = SpriteAnimation::from_state(state);
        self.player_animation_state = state;
        self
    }
}

/// The different state of the animation for the player
#[derive(Clone, Copy, Component, Default, Reflect, PartialEq)]
pub enum PlayerAnimationState {
    #[default]
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
