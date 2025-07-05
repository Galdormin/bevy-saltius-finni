//! Define codes to register and use button directly in cobweb files.

use bevy::{ecs::system::IntoObserverSystem, prelude::*};

use bevy_cobweb_ui::prelude::*;

use crate::states::{Menu, Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_button::<ChangeScreenButton>(change_screen)
        .register_button::<ChangeMenuButton>(change_menu)
        .register_button::<QuitButton>(quit_app);
}

/// Trait to register a button Component with a callback
///
/// # Example
/// ```
/// use bevy::prelude::*;
///
/// #[derive(Component, Debug, Default, Reflect, PartialEq)]
/// struct TestButton;
///
/// fn log_click(_: Trigger<Pointer<Click>>) {
///     info!("Button Pressed!");
/// }
///
/// App::new().register_button::<TestButton>(log_click);
/// ```
pub trait CobButtonRegistration<E: Event, B: Bundle, M> {
    fn register_button<T: Component + Loadable>(
        &mut self,
        observer: impl IntoObserverSystem<E, B, M> + Clone + Sync + 'static,
    ) -> &mut Self;
}

impl<E: Event, B: Bundle, M> CobButtonRegistration<E, B, M> for App {
    fn register_button<T: Component + Loadable>(
        &mut self,
        observer: impl IntoObserverSystem<E, B, M> + Clone + Sync + 'static,
    ) -> &mut Self {
        self.register_component_type::<T>().add_systems(
            Update,
            move |mut commands: Commands, buttons: Query<Entity, Added<T>>| {
                for entity in buttons.iter() {
                    commands.entity(entity).observe(observer.clone());
                }
            },
        )
    }
}

/// Cobweb component to change [`Screen`] on click
#[derive(Component, Debug, Default, Reflect, PartialEq)]
struct ChangeScreenButton(Screen);

/// Cobweb component to change [`Menu`] on click
#[derive(Component, Debug, Default, Reflect, PartialEq)]
struct ChangeMenuButton(Menu);

/// Cobweb component to quit on click
#[derive(Component, Debug, Default, Reflect, PartialEq)]
struct QuitButton;

fn quit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}

fn change_screen(
    trigger: Trigger<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
    buttons: Query<&ChangeScreenButton>,
) {
    if let Ok(button) = buttons.get(trigger.target) {
        next_screen.set(button.0);
    }
}

fn change_menu(
    trigger: Trigger<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    buttons: Query<&ChangeMenuButton>,
) {
    if let Ok(button) = buttons.get(trigger.target) {
        next_menu.set(button.0);
    }
}
