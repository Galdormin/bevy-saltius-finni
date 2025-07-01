use bevy::prelude::*;

use avian2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    GameLayer,
    platformer::entities::button::{ActivatedBy, ActivationStatus, UnresolvedActivateByRef},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<DoorBundle>("Door");
    app.add_systems(
        Update,
        (process_door, update_door_on_activation).run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct Door;

#[derive(Bundle, LdtkEntity)]
struct DoorBundle {
    door: Door,

    #[sprite_sheet]
    sprite: Sprite,

    #[with(UnresolvedActivateByRef::from_field)]
    unresolved_activate: UnresolvedActivateByRef,

    // Physics
    body: RigidBody,
    collider: Collider,
    collision_layer: CollisionLayers,
}

impl Default for DoorBundle {
    fn default() -> Self {
        Self {
            door: Door,
            sprite: Sprite::default(),
            unresolved_activate: UnresolvedActivateByRef::default(),
            body: RigidBody::Static,
            collider: Collider::rectangle(6.0, 32.0),
            collision_layer: CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        }
    }
}

fn process_door(
    mut commands: Commands,
    new_entity_instance: Query<(Entity, &mut Transform, &UnresolvedActivateByRef), Added<Door>>,
    ldtk_entities: Query<(Entity, &EntityIid)>,
) {
    for (entity, mut transform, unresolved_ref) in new_entity_instance {
        // Small offset to fix delta
        transform.translation.y += 4.0;

        if let Some(activater_iid) = unresolved_ref.0.as_ref() {
            let (activater_entity, _) = ldtk_entities
                .iter()
                .find(|(_, iid)| *iid == activater_iid)
                .expect("Activater entity should exists");

            commands
                .entity(entity)
                .remove::<UnresolvedActivateByRef>()
                .insert(ActivatedBy(activater_entity));
        } else {
            commands.entity(entity).remove::<UnresolvedActivateByRef>();
        }
    }
}

fn update_door_on_activation(
    mut commands: Commands,
    doors: Query<(Entity, &mut Sprite, &ActivationStatus), (With<Door>, Changed<ActivationStatus>)>,
) {
    for (entity, mut sprite, status) in doors {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            if status.0 {
                commands.entity(entity).insert(Sensor);
                atlas.index = 1;
            } else {
                commands.entity(entity).remove::<Sensor>();
                atlas.index = 0;
            };
        }
    }
}
