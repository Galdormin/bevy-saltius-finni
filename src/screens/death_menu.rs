//! Death menu to choose the genes

use bevy::prelude::*;
use bevy::ui::Val::*;

use sf_ui::prelude::{Menu, UiTheme, widget};

use crate::event::RespawnEvent;
use crate::player::genes::Gene;
use crate::player::genes::PlayerGenes;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<GeneContainer>();

    app.add_systems(OnEnter(Menu::Death), spawn_death_menu);
    app.add_systems(
        Update,
        (fill_gene_container, update_gene_container).run_if(in_state(Menu::Death)),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct GeneButton(usize);

#[derive(Component, Debug, Default, Reflect, PartialEq)]
#[reflect(Component)]
enum GeneContainer {
    #[default]
    Active,
    Inactive,
    Known,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
enum GeneType {
    Active,
    Inactive,
    Known,
}

impl From<&GeneContainer> for GeneType {
    fn from(value: &GeneContainer) -> Self {
        match value {
            GeneContainer::Active => GeneType::Active,
            GeneContainer::Inactive => GeneType::Inactive,
            GeneContainer::Known => GeneType::Known,
        }
    }
}

impl From<GeneContainer> for GeneType {
    fn from(value: GeneContainer) -> Self {
        match value {
            GeneContainer::Active => GeneType::Active,
            GeneContainer::Inactive => GeneType::Inactive,
            GeneContainer::Known => GeneType::Known,
        }
    }
}

fn spawn_death_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Death Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Death),
        children![
            widget::header("You have died!"),
            (
                Name::new("Gene Menu"),
                Node {
                    display: Display::Grid,
                    height: Percent(70.0),
                    row_gap: Px(3.0),
                    column_gap: Px(10.0),
                    grid_auto_flow: GridAutoFlow::Row,
                    grid_template_columns: RepeatedGridTrack::px(2, 100.0),
                    ..default()
                },
                children![
                    // Column headers
                    (
                        Name::new("Inactive Header"),
                        Node {
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        children![widget::label("Inactive")],
                    ),
                    (
                        Name::new("Active Header"),
                        Node {
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        children![widget::label("Active")],
                    ),
                    // Gene containers
                    (
                        Name::new("Inactive Genes"),
                        GeneContainer::Inactive,
                        Node {
                            height: Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                    ),
                    (
                        Name::new("Active Genes"),
                        GeneContainer::Active,
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                    ),
                ],
            ),
            widget::button("Respawn", respawn_on_click),
        ],
    ));
}

fn fill_gene_container(
    mut commands: Commands,
    player_genes: Res<PlayerGenes>,
    containers: Query<(Entity, &GeneContainer), Added<GeneContainer>>,
) {
    for (entity, container) in &containers {
        let genes = match container {
            GeneContainer::Active => player_genes.active_genes(),
            GeneContainer::Inactive => player_genes.inactive_genes(),
            GeneContainer::Known => player_genes.known_genes(),
        };

        for gene in genes {
            let gene = gene.to_owned();
            let gene_entity = commands
                .spawn((
                    Name::new(format!("Gene {}", gene.name)),
                    Button,
                    UiTheme::PIXEL_ART,
                    Node {
                        width: Px(80.0),
                        height: Px(13.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::vertical(Px(1.0)),
                        ..default()
                    },
                    ChildOf(entity),
                    GeneButton(gene.id),
                    GeneType::from(container),
                    children![(
                        Name::new("Gene Label"),
                        Text(gene.name.clone()),
                        UiTheme::PIXEL_ART,
                        TextFont::from_font_size(7.0),
                        TextColor(Color::WHITE),
                        Pickable::IGNORE,
                    )],
                ))
                .id();
            commands
                .entity(gene_entity)
                .observe(toggle_gene(gene.clone()));
        }
    }
}

fn update_gene_container(
    mut commands: Commands,
    containers: Query<(Entity, &GeneContainer)>,
    buttons: Query<(Entity, &GeneType), Changed<GeneType>>,
) {
    if buttons.is_empty() {
        return;
    }

    let Some((active_container, _)) = containers
        .iter()
        .find(|(_, c)| matches!(*c, GeneContainer::Active))
    else {
        return;
    };

    let Some((inactive_container, _)) = containers
        .iter()
        .find(|(_, c)| matches!(*c, GeneContainer::Inactive))
    else {
        return;
    };

    for (entity, gene_type) in &buttons {
        match gene_type {
            GeneType::Active => {
                commands.entity(entity).insert(ChildOf(active_container));
            }
            GeneType::Inactive => {
                commands.entity(entity).insert(ChildOf(inactive_container));
            }
            _ => (),
        }
    }
}

fn toggle_gene(
    gene: Gene,
) -> impl Fn(On<Pointer<Click>>, ResMut<PlayerGenes>, Query<&mut GeneType, With<GeneButton>>) {
    move |trigger, mut player_genes, mut gene_buttons| {
        let Ok(mut gene_type) = gene_buttons.get_mut(trigger.event().entity) else {
            return;
        };

        match *gene_type {
            GeneType::Active => {
                player_genes.remove_active_gene(gene.id);
                *gene_type = GeneType::Inactive;
            }
            GeneType::Inactive if player_genes.remaining_gene_slot() > 0 => {
                player_genes.add_active_gene(gene.id);
                *gene_type = GeneType::Active;
            }
            _ => (),
        }
    }
}

fn respawn_on_click(
    _: On<Pointer<Click>>,
    mut events: MessageWriter<RespawnEvent>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    events.write(RespawnEvent);
    next_menu.set(Menu::None);
}
