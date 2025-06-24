//! Code for the genes of the player

use bevy::{math::FloatPow, prelude::*};

use crate::player::movement::{JumpAmount, JumpImpulse, MovementSpeed};
use crate::player::physics::{CharacterController, GravityController};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, player_genes_changed);

    app.init_resource::<PlayerGenes>();
    app.insert_resource(PlayerGenes(vec![Gene {
        name: "Rabbit heritage".into(),
        stats: vec![
            GeneStat::JumpAmount(2),
            GeneStat::JumpHeight(5.0),
            GeneStat::MovementSpeed(8.0),
        ],
    }]));
}

/// Enum for the different Gene stats
#[derive(Clone, Debug, Reflect)]
pub enum GeneStat {
    JumpAmount(i32),    // Number of jumps before death
    JumpHeight(f32),    // Jump height in blocks (8 pixels)
    MovementSpeed(f32), // Movement speed in blocks/sec
    PowerUp,            // Power Up
}

impl Default for GeneStat {
    fn default() -> Self {
        GeneStat::JumpAmount(0)
    }
}

/// Struct that describe a gene
#[derive(Clone, Debug, Default, Reflect)]
pub struct Gene {
    name: String,
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
pub struct PlayerGenes(Vec<Gene>);

impl PlayerGenes {
    fn total_jump_amount(&self) -> i32 {
        self.0.iter().map(|gene| gene.get_jump_amount()).sum()
    }

    fn total_jump_height(&self) -> f32 {
        self.0.iter().map(|gene| gene.get_jump_height()).sum()
    }

    fn total_movement_speed(&self) -> f32 {
        self.0.iter().map(|gene| gene.get_movement_speed()).sum()
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
