//! Spawn the demo level for the platformer

use bevy::prelude::*;

use avian2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    camera::{LEVEL_HEIGHT, LEVEL_WIDTH, MainCamera},
    platformer::hud::JumpCounter,
    player::{
        movement::{Dead, JumpAmount, MovementBundle},
        physics::{CharacterController, CharacterControllerBundle},
    },
    screens::Screen,
    theme::palette::HEADER_TEXT,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();

    // LDTK
    app.insert_resource(LevelSelection::Uid(0));
    app.register_ldtk_int_cell::<WallBundle>(1);
    app.add_systems(
        Update,
        (spawn_wall, update_level_selection, restart_level).run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    world: Handle<LdtkProject>,
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            world: assets.load("world.ldtk"),
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

#[derive(Default, Component, Reflect, Debug)]
#[reflect(Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                Mesh2d(meshes.add(Capsule2d::new(4.0, 4.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
                Transform::from_xyz(LEVEL_WIDTH / 2.0, -LEVEL_HEIGHT / 2.0, 0.0),
                CharacterControllerBundle::new(Collider::capsule(4.0, 4.0))
                    .with_gravity(250.0, 350.0, 450.0),
                MovementBundle::default(),
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

fn spawn_wall(mut commands: Commands, walls: Query<Entity, Added<Wall>>) {
    for entity in walls {
        commands
            .entity(entity)
            .insert((Collider::rectangle(8.0, 8.0), RigidBody::Static));
    }
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

fn restart_level(
    mut commands: Commands,
    player: Single<(Entity, &mut Transform, &mut JumpAmount), With<CharacterController>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (entity, mut transform, mut jump_amount) = player.into_inner();
    if input.just_pressed(KeyCode::KeyR) {
        transform.translation = Vec3::new(LEVEL_WIDTH / 2.0, -LEVEL_HEIGHT / 2.0, 0.0);

        jump_amount.reset();
        commands.entity(entity).remove::<Dead>();
    }
}
