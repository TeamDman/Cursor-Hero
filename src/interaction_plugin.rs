use std::time::Duration;

use bevy::prelude::*;

use bevy::render::primitives::Aabb;
use bevy_spatial::{kdtree::KDTree2, SpatialAccess};
use bevy_spatial::{AutomaticUpdate, SpatialStructure};

use crate::update_ordering::InteractionSet;

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionEvent>()
            .add_plugins(
                AutomaticUpdate::<Interactable>::new()
                    .with_spatial_ds(SpatialStructure::KDTree2)
                    .with_frequency(Duration::from_millis(1)),
            )
            .add_systems(
                Update,
                (
                    reset_within_interaction_range_tag.in_set(InteractionSet::Rebuild),
                    update_within_interaction_range_tag.in_set(InteractionSet::Response),
                ),
            )
            .register_type::<WithinInteractionRange>()
            .register_type::<Interactable>()
            .register_type::<Interactor>();
    }
}

#[derive(Event)]
pub struct InteractionEvent(Entity, Entity);

#[derive(Component, Reflect, Default)]
pub struct Interactable;

#[derive(Component, Reflect, Default)]
pub struct Interactor;

#[derive(Component, Reflect)]
pub struct WithinInteractionRange {
    potential_interactor: Entity,
}

type NNTree = KDTree2<Interactable>;

fn reset_within_interaction_range_tag(
    mut commands: Commands,
    mut interactors: Query<Entity, With<WithinInteractionRange>>,
) {
    for entity in interactors.iter_mut() {
        commands.entity(entity).remove::<WithinInteractionRange>();
    }
}

fn update_within_interaction_range_tag(
    mut commands: Commands,
    mut interaction_events: EventWriter<InteractionEvent>,
    interactors: Query<(Entity, &Transform, &Aabb), With<Interactor>>,
    mut interactables: Query<(Entity, &Aabb), With<Interactable>>,
    tree: Res<NNTree>,
) {
    for (actor_entity, actor_pos, actor_hitbox) in interactors.iter() {
        for (_, close) in tree.within_distance(
            actor_pos.translation.xy(),
            actor_hitbox.half_extents.x.max(actor_hitbox.half_extents.y),
        ) {
            if let Some(close) = close {
                if let Ok(interactable) = interactables.get_mut(close) {
                    commands
                        .entity(interactable.0)
                        .insert(WithinInteractionRange {
                            potential_interactor: actor_entity,
                        });
                    interaction_events.send(InteractionEvent(actor_entity, close));
                }
            }
        }
    }
}
