//! Define Cobweb missing attributes

use bevy::{
    prelude::*,
    text::{ComputedTextBlock, TextLayoutInfo},
    ui::{ContentSize, widget::TextNodeFlags},
};

use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_static::<TextLineFont>()
        .register_static::<TextLineText>();
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
