//! Plugin to define different Cobweb components

use bevy::{
    prelude::*,
    text::{ComputedTextBlock, TextLayoutInfo},
    ui::{ContentSize, widget::TextNodeFlags},
};

use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;

use crate::{event::RespawnEvent, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_static::<TextLineFont>()
        .register_static::<TextLineText>();

    app.register_component_type::<ChangeScreenButton>()
        .register_component_type::<ChangeMenuButton>()
        .register_component_type::<QuitButton>()
        .register_component_type::<EventButton>();

    app.add_systems(
        Update,
        (
            add_observer_event_button,
            add_observer_change_screen_button,
            add_observer_change_menu_button,
            add_observer_quit_button,
        ),
    );
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

#[derive(Debug, Default, Reflect, PartialEq)]
enum EventType {
    #[default]
    None,
    Respawn,
}

/// Cobweb component to send an event on click
#[derive(Component, Debug, Default, Reflect, PartialEq)]
struct EventButton(EventType);

fn add_observer_change_screen_button(
    mut commands: Commands,
    buttons: Query<(Entity, &ChangeScreenButton), Added<ChangeScreenButton>>,
) {
    for (entity, screen) in buttons {
        let screen = screen.0;
        commands.entity(entity).observe(
            move |_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>| {
                next_screen.set(screen);
            },
        );
    }
}

fn add_observer_change_menu_button(
    mut commands: Commands,
    buttons: Query<(Entity, &ChangeMenuButton), Added<ChangeMenuButton>>,
) {
    for (entity, menu) in buttons {
        let menu = menu.0;
        commands.entity(entity).observe(
            move |_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>| {
                next_menu.set(menu);
            },
        );
    }
}

fn add_observer_quit_button(mut commands: Commands, buttons: Query<Entity, Added<QuitButton>>) {
    for entity in buttons {
        commands.entity(entity).observe(
            move |_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>| {
                app_exit.write(AppExit::Success);
            },
        );
    }
}

fn add_observer_event_button(
    mut commands: Commands,
    buttons: Query<(Entity, &EventButton), Added<EventButton>>,
) {
    for (entity, event) in buttons {
        if event.0 == EventType::Respawn {
            commands.entity(entity).observe(
                move |_: Trigger<Pointer<Click>>, mut ev: EventWriter<RespawnEvent>| {
                    info!("Test");
                    ev.write(RespawnEvent);
                },
            );
        }
    }
}
