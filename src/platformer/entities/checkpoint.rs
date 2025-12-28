use bevy::prelude::*;

use avian2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{GameLayer, platformer::level::Wall, player::death::RespawnPosition, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<CheckpointBundle>("Checkpoint");

    app.add_systems(
        Update,
        (
            process_checkpoint,
            (update_restart_position, update_checkpoint_sprite).chain(),
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct Checkpoint(bool);

impl Checkpoint {
    fn from_field(entity_instance: &EntityInstance) -> Self {
        let active = entity_instance
            .get_bool_field("active")
            .expect("Expected checlpoint to have active field");

        Self(*active)
    }
}

#[derive(Bundle, LdtkEntity)]
struct CheckpointBundle {
    #[with(Checkpoint::from_field)]
    checkpoint: Checkpoint,

    #[sprite_sheet]
    sprite: Sprite,
}

impl Default for CheckpointBundle {
    fn default() -> Self {
        Self {
            checkpoint: Checkpoint(false),
            sprite: Sprite::default(),
        }
    }
}

#[derive(Bundle)]
struct CheckpointBaseBundle {
    wall: Wall,
    body: RigidBody,
    collider: Collider,
    transform: Transform,
    collisions_layer: CollisionLayers,
}

impl Default for CheckpointBaseBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            body: RigidBody::Static,
            collider: Collider::rectangle(14.0, 4.0),
            transform: Transform::from_xyz(0.0, -6.0, 0.0),
            collisions_layer: CollisionLayers::new(GameLayer::Ground, [GameLayer::Player]),
        }
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct CheckpointScan;

#[derive(Bundle)]
struct CheckpointScanBundle {
    checkpoint: CheckpointScan,
    body: RigidBody,
    collider: Collider,
    sensor: Sensor,
    transform: Transform,
    collision_event: CollisionEventsEnabled,
    collisions_layer: CollisionLayers,
}

impl Default for CheckpointScanBundle {
    fn default() -> Self {
        Self {
            checkpoint: CheckpointScan,
            body: RigidBody::Static,
            collider: Collider::rectangle(2.0, 2.0),
            sensor: Sensor,
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            collision_event: CollisionEventsEnabled,
            collisions_layer: CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        }
    }
}

fn process_checkpoint(
    mut commands: Commands,
    new_entity_instance: Query<(Entity, &mut Transform), Added<Checkpoint>>,
) {
    for (entity, mut transform) in new_entity_instance {
        // Small offset to fix delta
        transform.translation.y += 4.0;

        // Spawn solid base
        commands.spawn((CheckpointBaseBundle::default(), ChildOf(entity)));
        // Spawn player detection
        commands
            .spawn((CheckpointScanBundle::default(), ChildOf(entity)))
            .observe(detect_checkpoint_activation);
    }
}

fn update_checkpoint_sprite(checkpoints: Query<(&mut Sprite, &Checkpoint), Changed<Checkpoint>>) {
    for (mut sprite, checkpoint) in checkpoints {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = if checkpoint.0 { 1 } else { 0 };
        }
    }
}

fn update_restart_position(
    checkpoints: Query<(&GlobalTransform, &Checkpoint), Changed<Checkpoint>>,
    mut respawn_position: ResMut<RespawnPosition>,
) {
    for (transform, checkpoint) in checkpoints {
        if checkpoint.0 {
            respawn_position.0 = transform.translation().truncate();
            return;
        }
    }
}

fn detect_checkpoint_activation(
    trigger: On<CollisionStart>,
    checkpoints_base: Query<&ChildOf, With<CheckpointScan>>,
    mut checkpoints: Query<(Entity, &mut Checkpoint)>,
) {
    if let Ok(checkpoint_entity) = checkpoints_base
        .get(trigger.event_target())
        .map(|ChildOf(parent)| parent)
    {
        checkpoints
            .iter_mut()
            .for_each(|(entity, mut checkpoint)| checkpoint.0 = entity == *checkpoint_entity);
    }
}
