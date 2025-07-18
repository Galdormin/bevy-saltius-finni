//! Spawn the demo level for the platformer

use bevy::{prelude::*, sprite::Anchor};

use avian2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    GameLayer,
    assets::collections::{LevelAssets, PlayerAssets},
    audio::music,
    camera::{LEVEL_HEIGHT, LEVEL_WIDTH, MainCamera},
    event::{DeathEvent, RespawnEvent},
    platformer::hud::JumpCounter,
    player::{
        animation::PlayerAnimationState,
        movement::{Dead, JumpAmount, MovementBundle, RespawnPosition},
        physics::{CharacterController, CharacterControllerBundle},
    },
    screens::Screen,
    ui::palette::HEADER_TEXT,
    utils::animation::SpriteAnimation,
};

pub(super) fn plugin(app: &mut App) {
    // LDTK
    app.insert_resource(LevelSelection::Uid(0));
    app.register_ldtk_int_cell::<WallBundle>(1);
    app.add_systems(
        Update,
        (
            update_level_selection,
            save_respawn,
            restart_level,
            respawn_player,
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Default, Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
    collider: Collider,
    collision_layers: CollisionLayers,
    body: RigidBody,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            collider: Collider::rectangle(8.0, 8.0),
            collision_layers: CollisionLayers::new(GameLayer::Ground, [GameLayer::Player]),
            body: RigidBody::Static,
        }
    }
}

pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            ),
            LdtkWorldBundle {
                ldtk_handle: level_assets.world.clone().into(),
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..Default::default()
            },
            (
                Sprite {
                    image: player_assets.sprite.clone(),
                    texture_atlas: Some(TextureAtlas::from(player_assets.atlas.clone())),
                    anchor: Anchor::Custom(Vec2::new(0.0, -0.2)),
                    ..default()
                },
                SpriteAnimation::from_state(PlayerAnimationState::Idle),
                PlayerAnimationState::Idle,
                Transform::from_xyz(LEVEL_WIDTH / 2.0, -LEVEL_HEIGHT / 2.0, 0.0),
                CharacterControllerBundle::new(Collider::capsule(4.0, 2.0))
                    .with_gravity(250.0, 350.0, 450.0),
                MovementBundle::default(),
                RespawnPosition(Vec2::new(LEVEL_WIDTH / 2.0, -LEVEL_HEIGHT / 2.0)),
            )
        ],
    ));
    commands.spawn((
        Name::new("HUD"),
        GlobalZIndex(1),
        StateScoped(Screen::Gameplay),
        Node::default(),
        children![(
            Text("0".into()),
            JumpCounter,
            TextFont::from_font_size(40.0),
            TextColor(HEADER_TEXT)
        )],
    ));
}

fn update_level_selection(
    levels: Query<(&LevelIid, &Transform), (Without<CharacterController>, Without<MainCamera>)>,
    player: Single<&Transform, (With<CharacterController>, Without<MainCamera>)>,
    mut camera: Single<&mut Transform, With<MainCamera>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_project_handle: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let ldtk_project = ldtk_project_assets
        .get(*ldtk_project_handle)
        .expect("Project should be loaded if level is spawned.");

    for (level_iid, level_transform) in &levels {
        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project.");

        let level_bounds = Rect {
            min: level_transform.translation.truncate(),
            max: level_transform.translation.truncate()
                + Vec2::new(level.px_wid as f32, level.px_hei as f32),
        };

        if !level_selection.is_match(&LevelIndices::default(), level)
            && level_bounds.contains(player.translation.truncate())
        {
            *level_selection = LevelSelection::iid(level.iid.clone());

            camera.translation = level_bounds.center().extend(camera.translation.z);
        }
    }
}

fn restart_level(mut death_event: EventWriter<DeathEvent>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyR) {
        death_event.write(DeathEvent);
    }
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
