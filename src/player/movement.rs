//! Code for the player movement (Jump, Gravity, Collision, etc.)

use bevy::prelude::*;

use avian2d::{math::*, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::{Action, AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_grounded, apply_gravity, movement)
            .chain()
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    )
    .add_systems(
        // Run collision handling after collision detection.
        //
        // NOTE: The collision implementation here is very basic and a bit buggy.
        //       A collide-and-slide algorithm would likely work better.
        PhysicsSchedule,
        kinematic_controller_collisions.in_set(NarrowPhaseSet::Last),
    );
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// The speed used for character movement.
#[derive(Component)]
pub struct MovementSpeed(pub Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(pub Scalar);

/// The gravitational acceleration used for a character controller.
#[derive(Component)]
pub struct GravityController {
    pub jump_gravity: Scalar,
    pub fall_gravity: Scalar,
    pub terminal_velocity: Scalar,
}

impl Default for GravityController {
    fn default() -> Self {
        Self {
            jump_gravity: 300.0,
            fall_gravity: 300.0,
            terminal_velocity: 200.0,
        }
    }
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    speed: MovementSpeed,
    jump_impulse: JumpImpulse,
    input_map: InputMap<Action>,
}

impl MovementBundle {
    pub fn new(speed: Scalar, jump_impulse: Scalar) -> Self {
        Self {
            speed: MovementSpeed(speed),
            jump_impulse: JumpImpulse(jump_impulse),
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

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    gravity: GravityController,
    movement: MovementBundle,
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            body: RigidBody::Kinematic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(1.0),
            gravity: GravityController::default(),
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(mut self, acceleration: Scalar, jump_impulse: Scalar) -> Self {
        self.movement = MovementBundle::new(acceleration, jump_impulse);
        self
    }

    pub fn with_gravity(
        mut self,
        jump_gravity: Scalar,
        fall_gravity: Scalar,
        terminal_velocity: Scalar,
    ) -> Self {
        self.gravity = GravityController {
            jump_gravity,
            fall_gravity,
            terminal_velocity,
        };
        self
    }
}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeHits), With<CharacterController>>,
) {
    for (entity, hits) in &mut query {
        // The character is grounded if the shape caster has a hit
        if hits.len() >= 1 {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Responds to [`Action`] events and moves character controllers accordingly.
fn movement(
    action_state: Single<&ActionState<Action>, With<CharacterController>>,
    controller: Single<
        (
            &MovementSpeed,
            &JumpImpulse,
            &mut LinearVelocity,
            Has<Grounded>,
        ),
        With<CharacterController>,
    >,
) {
    let (movement_speed, jump_impulse, mut linear_velocity, is_grounded) = controller.into_inner();

    if is_grounded && action_state.just_pressed(&Action::Jump) {
        linear_velocity.y = jump_impulse.0;
    }

    let mut direction = 0;
    for input in Action::DIRECTIONS {
        if action_state.pressed(&input) {
            if let Some(dir) = input.direction() {
                direction += dir;
            }
        }
    }

    linear_velocity.x = (direction as Scalar) * movement_speed.0;
}

/// Applies [`ControllerGravity`] to character controllers.
fn apply_gravity(
    time: Res<Time>,
    mut controllers: Query<(&GravityController, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for (gravity, mut linear_velocity) in &mut controllers {
        let gravity_force = if linear_velocity.y > 0.0 {
            gravity.jump_gravity
        } else {
            gravity.fall_gravity
        };

        linear_velocity.y -= gravity_force * delta_time;

        if linear_velocity.y.abs() > gravity.terminal_velocity {
            linear_velocity.y = -gravity.terminal_velocity;
        }
    }
}

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system handles collision response for kinematic character controllers
/// by pushing them along their contact normals by the current penetration depth,
/// and applying velocity corrections in order to snap to slopes, slide along walls,
/// and predict collisions using speculative contacts.
#[allow(clippy::type_complexity)]
fn kinematic_controller_collisions(
    collisions: Collisions,
    bodies: Query<&RigidBody>,
    collider_rbs: Query<&ColliderOf, Without<Sensor>>,
    mut character_controllers: Query<
        (&mut Position, &mut LinearVelocity),
        (With<RigidBody>, With<CharacterController>),
    >,
    time: Res<Time>,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([&ColliderOf { body: rb1 }, &ColliderOf { body: rb2 }]) =
            collider_rbs.get_many([contacts.collider1, contacts.collider2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;

        let character_rb: RigidBody;
        let is_other_dynamic: bool;

        let (mut position, mut linear_velocity) =
            if let Ok(character) = character_controllers.get_mut(rb1) {
                is_first = true;
                character_rb = *bodies.get(rb1).unwrap();
                is_other_dynamic = bodies.get(rb2).is_ok_and(|rb| rb.is_dynamic());
                character
            } else if let Ok(character) = character_controllers.get_mut(rb2) {
                is_first = false;
                character_rb = *bodies.get(rb2).unwrap();
                is_other_dynamic = bodies.get(rb1).is_ok_and(|rb| rb.is_dynamic());
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers.
        if !character_rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.normal
            } else {
                manifold.normal
            };

            let mut deepest_penetration: Scalar = Scalar::MIN;

            // Solve each penetrating contact in the manifold.
            for contact in manifold.points.iter() {
                if contact.penetration > 0.0 {
                    position.0 += normal * contact.penetration;
                }
                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static geometry.
            if is_other_dynamic {
                continue;
            }

            // Determine if the slope is climbable or if it's too steep to walk on.
            let slope_angle = normal.angle_to(Vector::Y);
            let climbable = slope_angle <= PI / 12.0;

            if deepest_penetration > 0.0 {
                // If the slope is climbable, snap the velocity so that the character
                // up and down the surface smoothly.
                if climbable {
                    // Points either left or right depending on which side the normal is leaning on.
                    // (This could be simplified for 2D, but this approach is dimension-agnostic)
                    let normal_direction_x =
                        normal.reject_from_normalized(Vector::Y).normalize_or_zero();

                    // The movement speed along the direction above.
                    let linear_velocity_x = linear_velocity.dot(normal_direction_x);

                    // Snap the Y speed based on the speed at which the character is moving
                    // up or down the slope, and how steep the slope is.
                    //
                    // A 2D visualization of the slope, the contact normal, and the velocity components:
                    //
                    //             ╱
                    //     normal ╱
                    // *         ╱
                    // │   *    ╱   velocity_x
                    // │       * - - - - - -
                    // │           *       | velocity_y
                    // │               *   |
                    // *───────────────────*

                    let max_y_speed = -linear_velocity_x * slope_angle.tan();
                    linear_velocity.y = linear_velocity.y.max(max_y_speed);
                } else {
                    // The character is intersecting an unclimbable object, like a wall.
                    // We want the character to slide along the surface, similarly to
                    // a collide-and-slide algorithm.

                    // Don't apply an impulse if the character is moving away from the surface.
                    if linear_velocity.dot(normal) > 0.0 {
                        continue;
                    }

                    // Slide along the surface, rejecting the velocity along the contact normal.
                    let impulse = linear_velocity.reject_from_normalized(normal);
                    linear_velocity.0 = impulse;
                }
            } else {
                // The character is not yet intersecting the other object,
                // but the narrow phase detected a speculative collision.
                //
                // We need to push back the part of the velocity
                // that would cause penetration within the next frame.

                let normal_speed = linear_velocity.dot(normal);

                // Don't apply an impulse if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply.
                let impulse_magnitude =
                    normal_speed - (deepest_penetration / time.delta_secs_f64().adjust_precision());
                let mut impulse = impulse_magnitude * normal;

                // Apply the impulse differently depending on the slope angle.
                if climbable {
                    // Avoid sliding down slopes.
                    linear_velocity.y -= impulse.y.min(0.0);
                } else {
                    // Avoid climbing up walls.
                    impulse.y = impulse.y.max(0.0);
                    linear_velocity.0 -= impulse;
                }
            }
        }
    }
}
