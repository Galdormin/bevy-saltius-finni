//! Death menu to choose the genes

use bevy::prelude::*;
use bevy::ui::Val::*;

use crate::player::genes::Gene;
use crate::{event::RespawnEvent, menus::Menu, player::genes::PlayerGenes, ui::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Menu::Death),
        (spawn_death_menu, fill_gene_container)
            .chain()
            .run_if(in_state(Menu::Death)),
    );

    app.add_systems(Update, update_gene_container.run_if(in_state(Menu::Death)));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct GeneButton(usize);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
enum GeneContainer {
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
        StateScoped(Menu::Death),
        children![
            widget::header("You have died!"),
            gene_choice(),
            widget::button("Respawn", respawn),
        ],
    ));
}

fn gene_choice() -> impl Bundle {
    (
        Name::new("Gene menu"),
        Node {
            display: Display::Grid,
            height: Percent(60.0),
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                Name::new("Inactive genes"),
                Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                GeneContainer::Inactive,
            ),
            (
                Name::new("Active genes"),
                Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                GeneContainer::Active,
            )
        ],
    )
}

fn respawn(
    _: Trigger<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut respawn_event: EventWriter<RespawnEvent>,
) {
    next_menu.set(Menu::None);
    respawn_event.write(RespawnEvent);
}

fn fill_gene_container(
    mut commands: Commands,
    player_genes: Res<PlayerGenes>,
    containers: Query<(Entity, &GeneContainer), Added<GeneContainer>>,
) {
    for (entity, container) in containers {
        let genes = match container {
            GeneContainer::Active => player_genes.active_genes(),
            GeneContainer::Inactive => player_genes.inactive_genes(),
            GeneContainer::Known => player_genes.known_genes(),
        };

        genes.iter().for_each(|gene| {
            commands.spawn((
                GeneButton(gene.id),
                GeneType::from(container),
                widget::button_small(gene.name.clone(), toggle_gene(gene.to_owned().clone())),
                ChildOf(entity),
            ));
        });
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

    for (entity, gene_type) in buttons {
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
) -> impl Fn(
    Trigger<Pointer<Click>>,
    ResMut<PlayerGenes>,
    Query<&mut GeneType, With<GeneButton>>,
    Query<&ChildOf, With<Button>>,
) {
    move |trigger, mut player_genes, mut gene_buttons, buttons| {
        let Ok(mut gene_type) = buttons
            .get(trigger.event().target)
            .and_then(|child| gene_buttons.get_mut(child.0))
        else {
            return;
        };

        match *gene_type {
            GeneType::Active => {
                player_genes.remove_active_gene(gene.id);
                *gene_type = GeneType::Inactive;
            }
            GeneType::Inactive => {
                if player_genes.remaining_gene_slot() > 0 {
                    player_genes.add_active_gene(gene.id);
                    *gene_type = GeneType::Active;
                }
            }
            _ => (),
        }
    }
}
