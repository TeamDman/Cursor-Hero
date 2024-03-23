use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_xpbd_2d::components::CollidingEntities;
use cursor_hero_environment_types::prelude::*;
use cursor_hero_level_bounds::level_bounds_plugin::LevelBounds;
use cursor_hero_level_bounds::level_bounds_plugin::LevelBoundsHolder;

pub struct EnvironmentTrackerPlugin;

impl Plugin for EnvironmentTrackerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, track);
    }
}

fn track(
    mut commands: Commands,
    mut thing_query: Query<(Entity, Option<&mut EnvironmentTracker>, &CollidingEntities)>,
    level_bounds_query: Query<&Parent, With<LevelBounds>>,
    level_bounds_holder_query: Query<&Parent, With<LevelBoundsHolder>>,
) {
    for (thing_id, thing_environment_tag, thing_colliding_entities) in thing_query.iter_mut() {
        // find out what level bounds the pointer is touching
        // find those bounds' parent
        // find the parent of the parent to get the environment ID
        let environment_ids = thing_colliding_entities
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
                "Thing {:?} is touching multiple environments: {:?}",
                thing_id, environment_ids
            );
        }
        if let Some(environment_id) = environment_ids.iter().next() {
            if let Some(mut tag) = thing_environment_tag {
                tag.environment_id = *environment_id;
            } else {
                commands.entity(thing_id).insert(EnvironmentTracker {
                    environment_id: *environment_id,
                });
            }
        } else if thing_environment_tag.is_some() {
            commands.entity(thing_id).remove::<EnvironmentTracker>();
        }
    }
}
