use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_xpbd_2d::components::CollidingEntities;
use cursor_hero_level_bounds::level_bounds_plugin::LevelBounds;
use cursor_hero_level_bounds::level_bounds_plugin::LevelBoundsHolder;
use cursor_hero_pointer_types::prelude::*;

pub struct PointerEnvironmentPlugin;

impl Plugin for PointerEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, track_pointer_environment);
    }
}

fn track_pointer_environment(
    mut commands: Commands,
    mut pointer_query: Query<
        (Entity, Option<&mut PointerEnvironment>, &CollidingEntities),
        With<Pointer>,
    >,
    level_bounds_query: Query<&Parent, With<LevelBounds>>,
    level_bounds_holder_query: Query<&Parent, With<LevelBoundsHolder>>,
) {
    for (pointer_id, pointer_environment, pointer_colliding_entities) in pointer_query.iter_mut() {
        // find out what level bounds the pointer is touching
        // find those bounds' parent
        // find the parent of the parent to get the environment ID
        let environment_ids = pointer_colliding_entities
            .0
            .iter()
            .filter_map(|entity| {
                if let Ok(level_bounds_holder_id) = level_bounds_query.get(*entity) {
                    if let Ok(environment_id) =
                        level_bounds_holder_query.get(level_bounds_holder_id.get())
                    {
                        return Some(environment_id.get());
                    }
                }
                None
            })
            .collect::<HashSet<Entity>>();
        if environment_ids.len() > 1 {
            warn!(
                "Pointer {:?} is touching multiple environments: {:?}",
                pointer_id, environment_ids
            );
        }
        if let Some(environment_id) = environment_ids.iter().next() {
            if let Some(mut pointer_environment) = pointer_environment {
                pointer_environment.environment_id = *environment_id;
            } else {
                commands.entity(pointer_id).insert(PointerEnvironment {
                    environment_id: *environment_id,
                });
            }
        } else if pointer_environment.is_some() {
            commands.entity(pointer_id).remove::<PointerEnvironment>();
        }
    }
}
