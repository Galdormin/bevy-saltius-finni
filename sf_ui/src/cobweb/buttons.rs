//! Define codes to register and use button directly in cobweb files.

use bevy::{
    ecs::system::IntoObserverSystem,
    prelude::*,
    reflect::{GetTypeRegistration, Typed},
    state::state::FreelyMutableState,
};

use bevy_cobweb_ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_button::<QuitButton>(quit_app);
}

/// Trait to register a button Component with a callback
pub trait CobButtonRegistration<E: Event, B: Bundle, M> {
    /// Register a new Cobweb Component with an observer?
    ///
    ///
    /// ## In Rust file
    ///
    /// ```
    /// use bevy::prelude::*;
    ///
    /// #[derive(Component, Debug, Default, Reflect, PartialEq)]
    /// struct MyButton(u32)
    ///
    /// fn print_value_on_click(trigger: Trigger<Pointer<Click>>, buttons: Query<&MyButton>) {
    ///     if let Ok(MyButton(value)) = buttons.get(trigger.target) {
    ///         info!("Value clicked: {}", value);
    ///     }
    /// }
    ///
    /// pub fn plugin(app: &mut App) {
    ///    app.register_button::<MyButton>(print_value_on_click);
    /// }
    /// ```
    ///
    /// ## In CobWeb file
    ///
    /// ```
    /// "button"
    ///     MyButton(312)
    /// ```
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

/// Trait to register a button Component that change a state
pub trait CubButtonStateRegistration {
    /// Register a new State that register a StateButton Component with an
    /// observer that change the given state.
    ///
    /// ## In Rust file
    ///
    /// ```
    /// use bevy::prelude::*;
    ///
    /// #[derive(Reflect, States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
    /// #[states(scoped_entities)]
    /// enum MyState {
    ///     #[default]
    ///     A,
    ///     B,
    /// }
    ///
    /// pub fn plugin(app: &mut App) {
    ///    app.register_button_state::<MyState>();
    /// }
    /// ```
    ///
    /// ## In CobWeb file
    ///
    /// ```
    /// "button"
    ///     StateButton<MyState>(B)
    /// ```
    fn register_button_state<S: ButtonStates>(&mut self) -> &mut Self;
}

impl CubButtonStateRegistration for App {
    fn register_button_state<S: ButtonStates>(&mut self) -> &mut Self {
        self.register_button::<StateButton<S>>(change_state::<S>)
    }
}

/// Trait implemented for State
pub trait ButtonStates:
    States
    + Reflect
    + FromReflect
    + Default
    + Typed
    + GetTypeRegistration
    + FreelyMutableState
    + Copy
    + Clone
{
}

impl<S> ButtonStates for S where
    S: States
        + Reflect
        + FromReflect
        + Default
        + Typed
        + GetTypeRegistration
        + FreelyMutableState
        + Copy
        + Clone
{
}

#[derive(Component, Debug, Default, Reflect, PartialEq)]
struct StateButton<S: ButtonStates>(S);

/// Cobweb component to quit on click
#[derive(Component, Debug, Default, Reflect, PartialEq)]
struct QuitButton;

fn quit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}

fn change_state<S: ButtonStates>(
    trigger: Trigger<Pointer<Click>>,
    mut next_state: ResMut<NextState<S>>,
    buttons: Query<&StateButton<S>>,
) {
    if let Ok(button) = buttons.get(trigger.target) {
        next_state.set(button.0);
    }
}
