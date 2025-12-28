use bevy::prelude::*;

use crate::{assets::collections::UiAssets, audio::sound_effect};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(Update, (apply_interaction_palette, apply_interaction_atlas));

    app.add_observer(play_on_hover_sound_effect);
    app.add_observer(play_on_click_sound_effect);
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

/// TextureAtlas for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`TextureAtlas`] index based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionAtlas {
    pub none: usize,
    pub hovered: usize,
    pub pressed: usize,
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

fn apply_interaction_atlas(
    mut palette_query: Query<
        (&Interaction, &InteractionAtlas, &mut ImageNode),
        Changed<Interaction>,
    >,
) {
    for (interaction, atlas, mut image_node) in &mut palette_query {
        let Some(texture_atlas) = image_node.texture_atlas.as_mut() else {
            continue;
        };

        texture_atlas.index = match interaction {
            Interaction::None => atlas.none,
            Interaction::Hovered => atlas.hovered,
            Interaction::Pressed => atlas.pressed,
        };
    }
}

fn play_on_hover_sound_effect(
    trigger: On<Pointer<Over>>,
    mut commands: Commands,
    ui_assets: Option<Res<UiAssets>>,
    interaction_query: Query<(), With<Interaction>>,
) {
    let Some(ui_assets) = ui_assets else {
        return;
    };

    if interaction_query.contains(trigger.entity) {
        commands.spawn(sound_effect(ui_assets.hover_sound.clone()));
    }
}

fn play_on_click_sound_effect(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    ui_assets: Option<Res<UiAssets>>,
    interaction_query: Query<(), With<Interaction>>,
) {
    let Some(ui_assets) = ui_assets else {
        return;
    };

    if interaction_query.contains(trigger.entity) {
        commands.spawn(sound_effect(ui_assets.click_sound.clone()));
    }
}
