//! Handle death of player

use avian2d::prelude::{Collider, CollisionLayers, RigidBody, SleepingDisabled};
use bevy::prelude::*;

use crate::{
    GameLayer,
    event::{DeathEvent, RespawnEvent},
    player::{
        movement::{JumpAmount, RespawnPosition},
        physics::{CharacterController, Grounded},
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            (update_dead, add_dead_on_death).chain(),
            save_respawn,
            (spawn_body_on_death, respawn_player).chain(),
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

/// A marker component indicating that the player is dead.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Dead;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct DeadBody;

#[derive(Default, Bundle)]
struct DeadBodyBundle {
    dead_body: DeadBody,
    sprite: Sprite,
    transform: Transform,

    // Physics
    body: RigidBody,
    collider: Collider,
    collision_layer: CollisionLayers,
}

/// Detect the last jump of the player and trigger "Dead" behavior
fn update_dead(
    mut death_event: EventWriter<DeathEvent>,
    jump_amount: Single<&JumpAmount, (Added<Grounded>, With<CharacterController>)>,
) {
    if jump_amount.remaining == 0 {
        death_event.write(DeathEvent);
    }
}

fn add_dead_on_death(
    mut commands: Commands,
    death_event: EventReader<DeathEvent>,
    player: Single<Entity, With<CharacterController>>,
) {
    if !death_event.is_empty() {
        commands.entity(*player).insert(Dead);
    }
}

fn spawn_body_on_death(
    mut commands: Commands,
    mut respawn_event: EventReader<RespawnEvent>,
    player: Single<(&Sprite, &Transform, &Collider, &ChildOf), With<CharacterController>>,
) {
    if respawn_event.is_empty() {
        return;
    }

    respawn_event.clear();

    let (player_sprite, player_transform, player_collider, player_childof) = *player;

    commands.spawn((
        DeadBody,
        player_sprite.clone(),
        player_transform.clone(),
        player_collider.clone(),
        player_childof.clone(),
        RigidBody::Kinematic,
        SleepingDisabled,
        CollisionLayers::new(GameLayer::Player, [GameLayer::Ground, GameLayer::Sensor]),
    ));
}

fn save_respawn(
    player: Single<(&Transform, &mut RespawnPosition), With<CharacterController>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyT) {
        let (transform, mut respawn_position) = player.into_inner();
        *respawn_position = RespawnPosition(transform.translation.truncate());
    }
}

fn respawn_player(
    mut commands: Commands,
    respawn_event: EventReader<RespawnEvent>,
    player: Single<
        (Entity, &mut Transform, &mut JumpAmount, &RespawnPosition),
        With<CharacterController>,
    >,
) {
    if respawn_event.is_empty() {
        return;
    }

    let (entity, mut transform, mut jump_amount, respawn_position) = player.into_inner();

    transform.translation = respawn_position.0.extend(0.0);

    jump_amount.reset();
    commands.entity(entity).remove::<Dead>();
}
