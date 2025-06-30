// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod assets;
mod audio;
mod camera;
#[cfg(feature = "dev")]
mod dev_tools;
mod event;
mod menus;
mod platformer;
mod player;
mod screens;
mod ui;
mod utils;

use bevy::{asset::AssetMetaCheck, prelude::*};

use avian2d::{PhysicsPlugins, prelude::PhysicsLayer};
use bevy_ecs_ldtk::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Bevy Saltius Finni".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
            PhysicsPlugins::default().with_length_unit(8.0),
            InputManagerPlugin::<Action>::default(),
            LdtkPlugin,
        ));

        // Add other plugins.
        app.add_plugins((
            assets::plugin,
            audio::plugin,
            camera::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            event::plugin,
            menus::plugin,
            player::plugin,
            platformer::plugin,
            screens::plugin,
            ui::plugin,
            utils::plugin,
        ));

        // Resources
        app.insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..Default::default()
        });

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    // Movement
    Left,
    Right,
    Jump,
}

impl Action {
    const DIRECTIONS: [Self; 2] = [Action::Left, Action::Right];

    fn direction(self) -> Option<i8> {
        match self {
            Action::Left => Some(-1),
            Action::Right => Some(1),
            _ => None,
        }
    }
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default, // Layer 0 - the default layer that objects are assigned to
    Player, // Layer 1
    Ground, // Layer 2
    Sensor, // Layer 3
}
