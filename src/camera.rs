//! Module with the definition of the camera
use bevy::prelude::*;

use bevy_modern_pixel_camera::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PixelCameraPlugin);

    // app.insert_resource(Msaa::Off);

    // Spawn the main camera.
    app.add_systems(Startup, spawn_camera);
}

pub const LEVEL_WIDTH: f32 = 320.0;
pub const LEVEL_HEIGHT: f32 = 180.0;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        MainCamera,
        PixelViewport,
        PixelZoom::FitSize {
            width: LEVEL_WIDTH as i32,
            height: LEVEL_HEIGHT as i32,
        },
        Transform::from_xyz(LEVEL_WIDTH / 2.0, -LEVEL_HEIGHT / 2.0, 0.0),
    ));
}
