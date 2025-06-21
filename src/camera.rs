//! Module with the definition of the camera
use bevy::prelude::*;

use bevy_modern_pixel_camera::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PixelCameraPlugin);

    // app.insert_resource(Msaa::Off);

    // Spawn the main camera.
    app.add_systems(Startup, spawn_camera);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        MainCamera,
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
        PixelViewport,
    ));
}
