//! Spawn the demo level for the platformer

use bevy::prelude::*;

use avian2d::{math::*, prelude::*};

use crate::{
    asset_tracking::LoadResource, audio::music, platformer::player::CharacterControllerBundle,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
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

/// A system that spawns the main level.
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
            platform(Vec2::new(280.0, 10.0), Vec2::new(0.0, -70.0)),
            platform(Vec2::new(30.0, 2.0), Vec2::new(-115.0, -45.0)),
            platform(Vec2::new(50.0, 2.0), Vec2::new(-50.0, -30.0)),
            platform(Vec2::new(30.0, 2.0), Vec2::new(115.0, -45.0)),
            platform(Vec2::new(50.0, 2.0), Vec2::new(50.0, -30.0)),
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

fn platform(size: Vec2, position: Vec2) -> impl Bundle {
    (
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_translation(position.extend(0.0)),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
    )
}
