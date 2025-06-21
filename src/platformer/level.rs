//! Spawn the demo level for the platformer

use bevy::prelude::*;

use avian2d::{math::*, prelude::*};
use bevy_ecs_ldtk::prelude::*;

use crate::{
    asset_tracking::LoadResource, audio::music, platformer::player::CharacterControllerBundle,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();

    // LDTK
    app.insert_resource(LevelSelection::index(0));
    app.register_ldtk_int_cell::<WallBundle>(1);
    app.add_systems(Update, spawn_wall.run_if(in_state(Screen::Gameplay)));
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
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
    asset_server: Res<AssetServer>,
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
                ldtk_handle: asset_server.load("world.ldtk").into(),
                transform: Transform::from_xyz(-160.0, -90.0, -1.0),
                ..Default::default()
            },
            (
                Mesh2d(meshes.add(Capsule2d::new(4.0, 4.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
                Transform::default(),
                CharacterControllerBundle::new(Collider::capsule(4.0, 4.0), Vector::NEG_Y * 300.0)
                    .with_movement(60.0, 120.0, (30.0 as Scalar).to_radians()),
            )
        ],
    ));
}

fn spawn_wall(mut commands: Commands, walls: Query<Entity, Added<Wall>>) {
    for entity in walls {
        commands
            .entity(entity)
            .insert((Collider::rectangle(8.0, 8.0), RigidBody::Static));
    }
}
