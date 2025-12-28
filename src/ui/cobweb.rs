//! Plugin to define different Cobweb components

use bevy::{
    ecs::system::IntoObserverSystem,
    prelude::*,
    text::{ComputedTextBlock, TextLayoutInfo},
    ui::{ContentSize, widget::TextNodeFlags},
};

use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;

use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_static::<TextLineFont>()
        .register_static::<TextLineText>();

    app.register_button::<ChangeScreenButton>(change_screen)
        .register_button::<ChangeMenuButton>(change_menu)
        .register_button::<QuitButton>(quit_app);
}

/* Cobweb missing component */

/// Instruction for setting the font of a [`TextLine`] on an entity.
#[derive(Reflect, Default, Debug, Clone, PartialEq)]
struct TextLineFont(FontRequest);

impl Instruction for TextLineFont {
    fn apply(self, entity: Entity, world: &mut World) {
        world.syscall(
            (entity, self.0),
            |In((entity, request)): In<(Entity, FontRequest)>,
             mut c: Commands,
             font_map: Res<FontMap>,
             mut editor: TextEditor| {
                // Load font
                let font = font_map.get(&request);

                if let Some((_, text_font, _)) = editor.root(entity) {
                    text_font.font = font;
                } else if let Ok(mut ec) = c.get_entity(entity) {
                    ec.try_insert((Text("[[text line]]".into()), TextFont { font, ..default() }));
                }
            },
        );
    }

    fn revert(entity: Entity, world: &mut World) {
        // NOTE: Can't use remove_with_requires because removing and reinserting ComputedNodeTarget breaks UI.
        let _ = world.get_entity_mut(entity).map(|mut e| {
            e.remove::<(
                Text,
                TextLayout,
                TextFont,
                TextColor,
                TextNodeFlags,
                ContentSize,
                ComputedTextBlock,
                TextLayoutInfo,
            )>();
        });
    }
}

impl StaticAttribute for TextLineFont {
    type Value = FontRequest;
    fn construct(value: Self::Value) -> Self {
        TextLineFont(value)
    }
}

/// Instruction for setting the text of a [`TextLine`] on an entity.
#[derive(Reflect, Default, Debug, Clone, PartialEq)]
struct TextLineText(String);

impl Instruction for TextLineText {
    fn apply(self, entity: Entity, world: &mut World) {
        world.syscall(
            (entity, self.0),
            |In((entity, text)): In<(Entity, String)>, mut c: Commands, mut editor: TextEditor| {
                if let Some((current_text, _, _)) = editor.root(entity) {
                    *current_text = text;
                } else if let Ok(mut ec) = c.get_entity(entity) {
                    ec.try_insert(Text(text));
                }
            },
        );
    }

    fn revert(entity: Entity, world: &mut World) {
        // NOTE: Can't use remove_with_requires because removing and reinserting ComputedNodeTarget breaks UI.
        let _ = world.get_entity_mut(entity).map(|mut e| {
            e.remove::<(
                Text,
                TextLayout,
                TextFont,
                TextColor,
                TextNodeFlags,
                ContentSize,
                ComputedTextBlock,
                TextLayoutInfo,
            )>();
        });
    }
}

impl StaticAttribute for TextLineText {
    type Value = String;
    fn construct(value: Self::Value) -> Self {
        TextLineText(value)
    }
}

pub trait CobButtonRegistration<E: bevy::prelude::EntityEvent, B: Bundle, M> {
    fn register_button<T: Component + Loadable>(
        &mut self,
        observer: impl IntoObserverSystem<E, B, M> + Clone + Sync + 'static,
    ) -> &mut Self;
}

impl<E: bevy::prelude::EntityEvent, B: Bundle, M> CobButtonRegistration<E, B, M> for App {
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

/* Custom Cobweb component */

/// Cobweb component to change [`Screen`] on click
#[derive(Component, Debug, Default, Reflect, PartialEq)]
struct ChangeScreenButton(Screen);

/// Cobweb component to change [`Menu`] on click
#[derive(Component, Debug, Default, Reflect, PartialEq)]
struct ChangeMenuButton(Menu);

/// Cobweb component to quit on click
#[derive(Component, Debug, Default, Reflect, PartialEq)]
struct QuitButton;

fn quit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}

fn change_screen(
    trigger: On<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
    buttons: Query<&ChangeScreenButton>,
) {
    if let Ok(button) = buttons.get(trigger.entity) {
        next_screen.set(button.0);
    }
}

fn change_menu(
    trigger: On<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    buttons: Query<&ChangeMenuButton>,
) {
    if let Ok(button) = buttons.get(trigger.entity) {
        next_menu.set(button.0);
    }
}
