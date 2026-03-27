//! Gene system types for Saltius Finni

use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

pub struct SfGenePlugin;

impl Plugin for SfGenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<GeneDatabase>::new(&["genes.ron"]));
        app.add_systems(Startup, load_gene_database);
        app.init_resource::<PlayerGenes>();
    }
}

#[derive(Asset, Clone, Debug, Deserialize, Reflect)]
pub struct GeneDatabase(pub Vec<Gene>);

#[derive(Resource)]
pub struct GeneDatabaseHandle(pub Handle<GeneDatabase>);

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

/// Struct that describes a gene
#[derive(Clone, Debug, Default, Deserialize, Reflect)]
pub struct Gene {
    pub id: usize,
    pub name: String,
    pub stats: Vec<GeneStat>,
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

    /// Format gene stats as a description string for UI display
    pub fn description(&self) -> String {
        self.stats
            .iter()
            .map(|s| match s {
                GeneStat::JumpAmount(v) => format!("Jumps: {:+}", v),
                GeneStat::JumpHeight(v) => format!("Jump Height: {:+.1}", v),
                GeneStat::MovementSpeed(v) => format!("Speed: {:+.1}", v),
                GeneStat::PowerUp(name) => format!("Power: {}", name),
            })
            .collect::<Vec<_>>()
            .join("\n")
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
    pub fn new(
        base: Gene,
        genes: Vec<usize>,
        max_active: usize,
        known: HashMap<usize, Gene>,
    ) -> Self {
        Self {
            base,
            genes,
            max_active,
            known,
        }
    }

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

    pub fn total_jump_amount(&self) -> i32 {
        self.all_active_genes()
            .iter()
            .map(|gene| gene.get_jump_amount())
            .sum()
    }

    pub fn total_jump_height(&self) -> f32 {
        self.all_active_genes()
            .iter()
            .map(|gene| gene.get_jump_height())
            .sum()
    }

    pub fn total_movement_speed(&self) -> f32 {
        self.all_active_genes()
            .iter()
            .map(|gene| gene.get_movement_speed())
            .sum()
    }
}

fn load_gene_database(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gene_database = GeneDatabaseHandle(asset_server.load("resources/genes.ron"));
    commands.insert_resource(gene_database);
}
