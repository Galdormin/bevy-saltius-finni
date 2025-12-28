//! Death menu to choose the genes

use bevy::prelude::*;

use bevy_cobweb_ui::prelude::*;

use crate::event::RespawnEvent;
use crate::player::genes::Gene;
use crate::ui::cobweb::CobButtonRegistration;
use crate::{menus::Menu, player::genes::PlayerGenes};

pub(super) fn plugin(app: &mut App) {
    app.load("ui/cobweb/death.cob");
    app.register_component_type::<GeneContainer>()
        .register_button::<RespawnButton>(send_respawn_event_on_click);

    app.add_systems(
        OnEnter(Menu::Death),
        (spawn_death_menu, fill_gene_container)
            .chain()
            .run_if(in_state(Menu::Death)),
    );

    app.add_systems(Update, update_gene_container.run_if(in_state(Menu::Death)));
}

#[derive(Component, Debug, Default, Reflect, PartialEq)]
struct RespawnButton;

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

fn spawn_death_menu(mut commands: Commands, mut scene_builder: SceneBuilder) {
    commands.ui_root().spawn_scene(
        ("ui/cobweb/death.cob", "scene"),
        &mut scene_builder,
        |handle| {
            handle.insert((DespawnOnExit(Menu::Death), GlobalZIndex(2)));
        },
    );
}

fn fill_gene_container(
    mut commands: Commands,
    mut scene_builder: SceneBuilder,
    player_genes: Res<PlayerGenes>,
    containers: Query<(Entity, &GeneContainer), Added<GeneContainer>>,
) {
    for (entity, container) in containers {
        let genes = match container {
            GeneContainer::Active => player_genes.active_genes(),
            GeneContainer::Inactive => player_genes.inactive_genes(),
            GeneContainer::Known => player_genes.known_genes(),
        };

        for gene in genes {
            commands.ui_root().spawn_scene(
                ("ui/cobweb/death.cob", "gene_button"),
                &mut scene_builder,
                |handle| {
                    handle.get("text").update_text(gene.name.clone());

                    handle
                        .insert((
                            Button,
                            ChildOf(entity),
                            GeneButton(gene.id),
                            GeneType::from(container),
                        ))
                        .observe(toggle_gene(gene.to_owned().clone()));
                },
            );
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

fn send_respawn_event_on_click(_: On<Pointer<Click>>, mut msg: MessageWriter<RespawnEvent>) {
    msg.write(RespawnEvent);
}
