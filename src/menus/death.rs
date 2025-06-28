//! Death menu to choose the genes

use bevy::ui::Val::*;
use bevy::{ecs::spawn::SpawnIter, prelude::*};

use crate::player::genes::Gene;
use crate::{event::RespawnEvent, menus::Menu, player::genes::PlayerGenes, ui::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Death), spawn_death_menu);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct GeneButton(usize);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct InactiveGeneContainer;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ActiveGeneContainer;

fn spawn_death_menu(mut commands: Commands, player_genes: Res<PlayerGenes>) {
    commands.spawn((
        widget::ui_root("Death Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Death),
        children![
            widget::header("You have died!"),
            gene_choice(&player_genes),
            widget::button("Respawn", respawn),
        ],
    ));
}

fn gene_choice(player_genes: &PlayerGenes) -> impl Bundle {
    let inactive_genes: Vec<Gene> = player_genes
        .inactive_genes()
        .iter()
        .map(|g| g.to_owned().clone())
        .collect();

    let active_genes: Vec<Gene> = player_genes
        .active_genes()
        .iter()
        .map(|g| g.to_owned().clone())
        .collect();

    (
        Name::new("Gene menu"),
        Node {
            display: Display::Grid,
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
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                InactiveGeneContainer,
                Children::spawn(SpawnIter(
                    inactive_genes
                        .into_iter()
                        .map(|gene| gene_inactive_button(gene))
                ))
            ),
            (
                Name::new("Active genes"),
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ActiveGeneContainer,
                Children::spawn(SpawnIter(
                    active_genes
                        .into_iter()
                        .map(|gene| gene_active_button(gene))
                ))
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

fn gene_inactive_button(gene: Gene) -> impl Bundle {
    (
        GeneButton(gene.id),
        widget::button(gene.name.clone(), add_gene(gene)),
    )
}

fn gene_active_button(gene: Gene) -> impl Bundle {
    (
        GeneButton(gene.id),
        widget::button(gene.name.clone(), remove_gene(gene)),
    )
}

fn add_gene(
    gene: Gene,
) -> impl Fn(
    Trigger<Pointer<Click>>,
    Commands,
    ResMut<PlayerGenes>,
    Single<Entity, With<ActiveGeneContainer>>,
    Query<(Entity, &GeneButton)>,
) {
    move |_, mut commands, mut player_genes, active_container, gene_buttons| {
        info!("Add Gene: {}", gene.name);

        // Find the GeneButton Entity
        let Some((gene_entity, _)) = gene_buttons.iter().find(|(_, button)| button.0 == gene.id)
        else {
            return;
        };

        player_genes.add_active_gene(gene.id);

        // Delete the button
        commands.entity(gene_entity).despawn();

        // Add a new GeneButton to the InactiveContainer
        commands.spawn((gene_active_button(gene.clone()), ChildOf(*active_container)));
    }
}

fn remove_gene(
    gene: Gene,
) -> impl Fn(
    Trigger<Pointer<Click>>,
    Commands,
    ResMut<PlayerGenes>,
    Single<Entity, With<InactiveGeneContainer>>,
    Query<(Entity, &GeneButton)>,
) {
    move |_, mut commands, mut player_genes, inactive_container, gene_buttons| {
        info!("Remove Gene: {}", gene.name);

        // Find the GeneButton Entity
        let Some((gene_entity, _)) = gene_buttons.iter().find(|(_, button)| button.0 == gene.id)
        else {
            return;
        };

        player_genes.remove_active_gene(gene.id);

        // Delete the button
        commands.entity(gene_entity).despawn();

        // Add a new GeneButton to the InactiveContainer
        commands.spawn((
            gene_inactive_button(gene.clone()),
            ChildOf(*inactive_container),
        ));
    }
}
