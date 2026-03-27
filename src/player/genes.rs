//! Player-specific gene systems

use bevy::platform::collections::HashMap;
use bevy::{math::FloatPow, prelude::*};

use sf_gene::{GeneDatabase, GeneDatabaseHandle, PlayerGenes};
use sf_ui::prelude::Screen;

use crate::player::movement::{JumpAmount, JumpImpulse, MovementSpeed};
use crate::player::physics::{CharacterController, GravityController};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, player_genes_changed);
    app.add_systems(OnEnter(Screen::Title), load_default_gene);
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

fn load_default_gene(
    mut commands: Commands,
    gene_db: Res<GeneDatabaseHandle>,
    mut gene_databases: ResMut<Assets<GeneDatabase>>,
) {
    if let Some(gene_db) = gene_databases.remove(gene_db.0.id()) {
        commands.insert_resource(PlayerGenes::new(
            gene_db.0[0].clone(),
            vec![1],
            2,
            HashMap::from([
                (1, gene_db.0[1].clone()),
                (2, gene_db.0[2].clone()),
                (3, gene_db.0[3].clone()),
                (4, gene_db.0[4].clone()),
            ]),
        ));
    }
}
