//! Handle death of player

use bevy::prelude::*;

use avian2d::{
    math::Vector,
    prelude::{Collider, CollisionLayers, RigidBody, SleepingDisabled},
};

use sf_ui::prelude::Screen;

use crate::{
    GameLayer,
    event::{DeathEvent, RespawnEvent},
    player::{
        movement::JumpAmount,
        physics::{CharacterController, Grounded},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(RespawnPosition(Vec2::ZERO));

    app.add_systems(
        Update,
        (
            (update_dead, add_dead_on_death).chain(),
            (spawn_body_on_death, respawn_player).chain(),
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

/// The position of the Respawn
#[derive(Resource, Reflect, Debug)]
pub struct RespawnPosition(pub Vector);

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
        *player_transform,
        player_collider.clone(),
        player_childof.clone(),
        RigidBody::Kinematic,
        SleepingDisabled,
        CollisionLayers::new(GameLayer::Player, [GameLayer::Ground, GameLayer::Sensor]),
    ));
}

fn respawn_player(
    mut commands: Commands,
    respawn_event: EventReader<RespawnEvent>,
    respawn_position: Res<RespawnPosition>,
    player: Single<(Entity, &mut Transform, &mut JumpAmount), With<CharacterController>>,
) {
    if respawn_event.is_empty() {
        return;
    }

    let (entity, mut transform, mut jump_amount) = player.into_inner();

    transform.translation = respawn_position.0.extend(0.0);

    jump_amount.reset();
    commands.entity(entity).remove::<Dead>();
}
