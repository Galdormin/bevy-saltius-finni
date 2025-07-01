use bevy::prelude::*;

use crate::{
    assets::collections::UiAssets,
    ui::{
        interaction::InteractionAtlas,
        palette::{BUTTON_BACKGROUND, BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND},
        prelude::InteractionPalette,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PreUpdate,
        (update_button_theme, update_font_theme).run_if(resource_exists::<UiAssets>),
    );
}

/// Theme for ui with all sub theme
#[derive(Component, Clone, Debug, Default)]
pub struct UiTheme {
    button: ButtonTheme,
    font: FontTheme,
}

impl UiTheme {
    pub const PIXEL_ART: Self = Self {
        button: ButtonTheme::PixelArt,
        font: FontTheme::Monogram,
    };
}

/// Theme for the Button texture
#[derive(Clone, Debug, Default)]
pub enum ButtonTheme {
    #[default]
    Simple,
    PixelArt,
}

/// Theme for the font
#[derive(Clone, Debug, Default)]
pub enum FontTheme {
    #[default]
    Default,
    M6x11,
    Monogram,
}

impl FontTheme {
    fn get_handle_from_assets(&self, ui_assets: &UiAssets) -> Handle<Font> {
        match self {
            FontTheme::Default => Handle::<Font>::default(),
            FontTheme::M6x11 => ui_assets.m6x11.clone(),
            FontTheme::Monogram => ui_assets.monogram.clone(),
        }
    }
}

fn update_button_theme(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    entities: Query<(Entity, &UiTheme), (Added<UiTheme>, With<Button>)>,
) {
    for (entity, theme) in entities {
        let mut entity_cmd = commands.entity(entity);
        match theme.button {
            ButtonTheme::Simple => entity_cmd.insert((
                BackgroundColor(BUTTON_BACKGROUND),
                InteractionPalette {
                    none: BUTTON_BACKGROUND,
                    hovered: BUTTON_HOVERED_BACKGROUND,
                    pressed: BUTTON_PRESSED_BACKGROUND,
                },
            )),
            ButtonTheme::PixelArt => entity_cmd.insert((
                ui_assets.button_image_node(),
                InteractionAtlas {
                    none: 0,
                    hovered: 1,
                    pressed: 1,
                },
            )),
        };
    }
}

fn update_font_theme(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    entities: Query<(Entity, &UiTheme, Option<&mut TextFont>), Added<UiTheme>>,
) {
    for (entity, theme, maybe_textfont) in entities {
        let font = theme.font.get_handle_from_assets(&ui_assets);

        if let Some(mut textfont) = maybe_textfont {
            textfont.font = font;
        } else {
            commands.entity(entity).insert(TextFont::from_font(font));
        }
    }
}
