//! Generic code for Sprite animation

use std::time::Duration;

use bevy::prelude::*;

use crate::AppSystems;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_animation_timer.in_set(AppSystems::TickTimers),
    );
}

pub trait AnimationState: Clone + Component {
    /// Return the number of frames for the animation
    fn get_frames(&self) -> usize;

    /// Return the index of the first frame of the animation
    fn get_start_frame(&self) -> usize;

    /// Return the duration of each frame of the animation
    /// None if no animation
    fn get_duration(&self) -> Option<Duration>;
}

/// Component that update the sprite to have an animation.
///
#[derive(Default, Component, Reflect)]
pub struct SpriteAnimation {
    timer: Option<Timer>,
    frame: usize,
    max_frame: usize,
}

impl SpriteAnimation {
    pub fn from_state(state: impl AnimationState) -> Self {
        let timer = state
            .get_duration()
            .map(|d| Timer::new(d, TimerMode::Repeating));

        Self {
            timer,
            frame: 0,
            max_frame: state.get_frames(),
        }
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        if let Some(timer) = self.timer.as_mut() {
            timer.tick(delta);

            if timer.finished() {
                self.frame = (self.frame + 1) % self.max_frame;
            }
        }
    }
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut SpriteAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the texture atlas to reflect changes in the animation.
pub fn update_animation_atlas<T: AnimationState>(
    mut query: Query<(&SpriteAnimation, &mut Sprite, &T)>,
) {
    for (animation, mut sprite, state) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };

        let frame_index = animation.frame + state.get_start_frame();
        if atlas.index != frame_index {
            atlas.index = frame_index;
        }
    }
}

/// Update SpriteAnimation when state change
pub fn update_sprite_animation<T: AnimationState>(
    query: Query<(&mut SpriteAnimation, &T), Changed<T>>,
) {
    for (mut animation, state) in query {
        *animation = SpriteAnimation::from_state(state.clone());
    }
}
