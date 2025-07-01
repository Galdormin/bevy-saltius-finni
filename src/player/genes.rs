//! Code for the genes of the player

use bevy::platform::collections::HashMap;
use bevy::{math::FloatPow, prelude::*};

use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::player::movement::{JumpAmount, JumpImpulse, MovementSpeed};
use crate::player::physics::{CharacterController, GravityController};
use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<GeneDatabase>::new(&["genes.ron"]));

    app.add_systems(Startup, load_gene_database);
    app.add_systems(Update, player_genes_changed);
    app.add_systems(OnEnter(Screen::Title), load_default_gene); // Change

    app.init_resource::<PlayerGenes>();
}

#[derive(Asset, Clone, Debug, Deserialize, Reflect)]
pub struct GeneDatabase(Vec<Gene>);

#[derive(Resource)]
pub struct GeneDatabaseHandle(Handle<GeneDatabase>);

/// Enum for the different Gene stats
#[derive(Clone, Debug, Deserialize, Reflect)]
pub enum GeneStat {
    JumpAmount(i32),    // Number of jumps before death
    JumpHeight(f32),    // Jump height in blocks (8 pixels)
    MovementSpeed(f32), // Movement speed in blocks/sec
    PowerUp(String),    // Power Up
}

impl Default for GeneStat {
    fn default() -> Self {
        GeneStat::JumpAmount(0)
    }
}

/// Struct that describe a gene
#[derive(Clone, Debug, Default, Deserialize, Reflect)]
pub struct Gene {
    pub id: usize,
    pub name: String,
    stats: Vec<GeneStat>,
}

impl Gene {
    pub fn get_jump_amount(&self) -> i32 {
        self.stats
            .iter()
            .filter_map(|stat| {
                if let GeneStat::JumpAmount(amount) = stat {
                    Some(amount)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn get_jump_height(&self) -> f32 {
        self.stats
            .iter()
            .filter_map(|stat| {
                if let GeneStat::JumpHeight(amount) = stat {
                    Some(amount)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn get_movement_speed(&self) -> f32 {
        self.stats
            .iter()
            .filter_map(|stat| {
                if let GeneStat::MovementSpeed(amount) = stat {
                    Some(amount)
                } else {
                    None
                }
            })
            .sum()
    }
}

/// Resource with all the genes of the character
#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct PlayerGenes {
    base: Gene,
    genes: Vec<usize>,
    max_active: usize,
    known: HashMap<usize, Gene>,
}

impl PlayerGenes {
    pub fn remaining_gene_slot(&self) -> usize {
        self.max_active - self.genes.len()
    }

    pub fn add_active_gene(&mut self, gene_id: usize) {
        if self.genes.len() < self.max_active {
            self.genes.push(gene_id);
        }
    }

    pub fn remove_active_gene(&mut self, gene_id: usize) {
        if let Some(index) = self.genes.iter().position(|gid| *gid == gene_id) {
            self.genes.remove(index);
        }
    }

    /// Return all known genes
    pub fn known_genes(&self) -> Vec<&Gene> {
        self.known.values().collect()
    }

    /// Return active genes
    pub fn active_genes(&self) -> Vec<&Gene> {
        self.genes
            .iter()
            .filter_map(|id| self.known.get(id))
            .collect::<Vec<&Gene>>()
    }

    /// Return known but inactive genes
    pub fn inactive_genes(&self) -> Vec<&Gene> {
        self.known
            .iter()
            .filter_map(|(id, gene)| {
                if self.genes.contains(id) {
                    None
                } else {
                    Some(gene)
                }
            })
            .collect::<Vec<&Gene>>()
    }

    /// Return active genes + base gene
    pub fn all_active_genes(&self) -> Vec<&Gene> {
        let mut genes = self.active_genes();
        genes.push(&self.base);
        genes
    }

    fn total_jump_amount(&self) -> i32 {
        self.all_active_genes()
            .iter()
            .map(|gene| gene.get_jump_amount())
            .sum()
    }

    fn total_jump_height(&self) -> f32 {
        self.all_active_genes()
            .iter()
            .map(|gene| gene.get_jump_height())
            .sum()
    }

    fn total_movement_speed(&self) -> f32 {
        self.all_active_genes()
            .iter()
            .map(|gene| gene.get_movement_speed())
            .sum()
    }
}

fn player_genes_changed(
    player_genes: Res<PlayerGenes>,
    player: Single<
        (
            &mut JumpAmount,
            &mut JumpImpulse,
            &mut MovementSpeed,
            &mut GravityController,
        ),
        With<CharacterController>,
    >,
) {
    if !player_genes.is_changed() {
        return;
    }

    let (mut jump_amount, mut jump_impulse, mut movement_speed, mut controller_gravity) =
        player.into_inner();

    jump_amount.max = player_genes.total_jump_amount().max(0) as u32;
    jump_amount.reset();

    movement_speed.0 = player_genes.total_movement_speed() * 8.0;

    // Compute the jump gravity and the jump impulse from jump time and jump height
    let jump_height = player_genes.total_jump_height();
    let jump_time = jump_height / 10.0;

    controller_gravity.jump_gravity = (2.0 * jump_height * 8.0) / jump_time.squared();
    jump_impulse.0 = controller_gravity.jump_gravity * jump_time;
}

fn load_gene_database(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gene_database = GeneDatabaseHandle(asset_server.load("resources/genes.ron"));
    commands.insert_resource(gene_database);
}

fn load_default_gene(
    mut commands: Commands,
    gene_db: Res<GeneDatabaseHandle>,
    mut gene_databases: ResMut<Assets<GeneDatabase>>,
) {
    if let Some(gene_db) = gene_databases.remove(gene_db.0.id()) {
        commands.insert_resource(PlayerGenes {
            base: gene_db.0[0].clone(),
            genes: vec![1],
            max_active: 2,
            known: HashMap::from([
                (1, gene_db.0[1].clone()),
                (2, gene_db.0[2].clone()),
                (3, gene_db.0[3].clone()),
                (4, gene_db.0[4].clone()),
            ]),
        });
    }
}
