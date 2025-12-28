//! Code for the player movement (Jump, Gravity, Collision, etc.)

use bevy::prelude::*;

use avian2d::{math::*, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::event::JumpEvent;
use crate::player::death::Dead;
use crate::player::physics::{CharacterController, Grounded};
use crate::{Action, AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_coyote_timer, movement, jump)
            .chain()
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// The speed used for character movement.Oh
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct MovementSpeed(pub Scalar);

/// The strength of a jump.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct JumpImpulse(pub Scalar);

/// The coyote timer of the Jump
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct CoyoteTimer(Timer);

impl Default for CoyoteTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

impl CoyoteTimer {
    fn can_jump(&self) -> bool {
        !self.0.is_finished()
    }

    fn reset_timer(&mut self) {
        self.0.reset();
    }
}

/// The amount of jump that can do the player.
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct JumpAmount {
    pub max: u32,
    pub remaining: u32,
}

impl JumpAmount {
    pub fn reset(&mut self) {
        self.remaining = self.max;
    }
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    speed: MovementSpeed,
    jump_impulse: JumpImpulse,
    jump_amount: JumpAmount,
    coyote_timer: CoyoteTimer,
    input_map: InputMap<Action>,
}

impl MovementBundle {
    pub fn new(speed: Scalar, jump_impulse: Scalar) -> Self {
        Self {
            speed: MovementSpeed(speed),
            jump_impulse: JumpImpulse(jump_impulse),
            jump_amount: JumpAmount::default(),
            coyote_timer: CoyoteTimer::default(),
            input_map: MovementBundle::default_input_map(),
        }
    }

    pub fn default_input_map() -> InputMap<Action> {
        use crate::Action::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Left, KeyCode::ArrowLeft);
        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Left, GamepadButton::DPadLeft);

        input_map.insert(Right, KeyCode::ArrowRight);
        input_map.insert(Right, KeyCode::KeyD);
        input_map.insert(Right, GamepadButton::DPadRight);

        input_map.insert(Jump, KeyCode::Space);
        input_map.insert(Jump, GamepadButton::South);

        input_map
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 7.0)
    }
}

/// Responds to [`Action`] events and moves character controllers accordingly.
fn movement(
    mut jump_event_writer: MessageWriter<JumpEvent>,
    action_state: Single<&ActionState<Action>, With<CharacterController>>,
    controller: Single<
        (
            &MovementSpeed,
            &mut LinearVelocity,
            &CoyoteTimer,
            Has<Grounded>,
            Has<Dead>,
        ),
        With<CharacterController>,
    >,
) {
    let (movement_speed, mut linear_velocity, coyote_timer, is_grounded, is_dead) =
        controller.into_inner();

    if is_dead {
        linear_velocity.x = 0.0;
        return;
    }

    if action_state.just_pressed(&Action::Jump) && (is_grounded || coyote_timer.can_jump()) {
        jump_event_writer.write(JumpEvent);
    }

    let mut direction = 0;
    for input in Action::DIRECTIONS {
        if action_state.pressed(&input)
            && let Some(dir) = input.direction()
        {
            direction += dir;
        }
    }

    linear_velocity.x = (direction as Scalar) * movement_speed.0;
}

/// Update the coyote timer every frame
fn update_coyote_timer(time: Res<Time>, players: Query<(&mut CoyoteTimer, Has<Grounded>)>) {
    for (mut coyote_timer, is_grounded) in players {
        if is_grounded {
            coyote_timer.reset_timer();
        } else {
            coyote_timer.0.tick(time.delta());
        }
    }
}

/// Handle Jump Behavior
fn jump(
    mut jump_event_reader: MessageReader<JumpEvent>,
    player: Single<(&mut JumpAmount, &JumpImpulse, &mut LinearVelocity), With<CharacterController>>,
) {
    let (mut jump_amount, jump_impulse, mut linear_velocity) = player.into_inner();
    for _ in jump_event_reader.read() {
        linear_velocity.0.y = jump_impulse.0;

        if jump_amount.remaining > 0 {
            jump_amount.remaining -= 1;
        }
    }
}
