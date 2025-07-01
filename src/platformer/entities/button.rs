use bevy::prelude::*;

use avian2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{GameLayer, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PushButtonBundle>("Button");
    app.add_systems(
        Update,
        (
            process_button,
            (
                detect_button_press,
                update_sprite_push_button,
                update_activation_devices,
            )
                .chain(),
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

/// Relationship component to represent the device is activated by
#[derive(Component, Deref)]
#[relationship(relationship_target = Activate)]
#[require(ActivationStatus)]
pub struct ActivatedBy(pub Entity);

/// Relationship component to represent all device it activate
#[derive(Component)]
#[relationship_target(relationship = ActivatedBy)]
pub struct Activate(Vec<Entity>);

impl Activate {
    pub fn entities(&self) -> &Vec<Entity> {
        &self.0
    }
}

/// Component to track the activation status
#[derive(Component, Debug, Default)]
pub struct ActivationStatus(pub bool);

/// Component to handle unresolved relationship at ldtk startup
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct UnresolvedActivateByRef(pub Option<EntityIid>);

impl UnresolvedActivateByRef {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        Self(
            entity_instance
                .get_maybe_entity_ref_field("activater")
                .expect("Expected entity to have activate entity ref field")
                .as_ref()
                .map(|entity_ref| EntityIid::new(entity_ref.entity_iid.clone())),
        )
    }
}

#[derive(Component, Debug, Default)]
pub struct PushButton(pub bool);

#[derive(Bundle, LdtkEntity)]
struct PushButtonBundle {
    button: PushButton,
    #[sprite_sheet]
    sprite: Sprite,

    // Physics
    body: RigidBody,
    collider: Collider,
    sensor: Sensor,
    collision_layer: CollisionLayers,
    collision_event: CollisionEventsEnabled,
    colliding_entities: CollidingEntities,
}

impl Default for PushButtonBundle {
    fn default() -> Self {
        Self {
            button: PushButton(false),
            sprite: Sprite::default(),
            body: RigidBody::Static,
            collider: Collider::rectangle(4.0, 2.0),
            sensor: Sensor,
            collision_layer: CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
            collision_event: CollisionEventsEnabled,
            colliding_entities: CollidingEntities::default(),
        }
    }
}

fn process_button(new_entity_instance: Query<&mut Transform, Added<PushButton>>) {
    for mut transform in new_entity_instance {
        // Small offset to fix delta
        transform.translation.y += 3.0;
    }
}

fn detect_button_press(mut query: Query<(&CollidingEntities, &mut PushButton)>) {
    for (colliding_entities, mut button) in &mut query {
        if colliding_entities.0.is_empty() == button.0 {
            button.0 = !button.0;
        }
    }
}

fn update_sprite_push_button(query: Query<(&mut Sprite, &PushButton), Changed<PushButton>>) {
    for (mut sprite, button) in query {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = if button.0 { 1 } else { 0 };
        }
    }
}

fn update_activation_devices(
    buttons: Query<(&PushButton, &Activate), Changed<PushButton>>,
    mut devices: Query<&mut ActivationStatus, With<ActivatedBy>>,
) {
    for (button, activated_devices) in buttons {
        for device_entity in activated_devices.entities() {
            let _ = devices.get_mut(*device_entity).map(|mut status| {
                status.0 = button.0;
                ()
            });
        }
    }
}
