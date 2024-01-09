use bevy::prelude::*;
use bevy_xpbd_2d::components::CollidingEntities;
use bevy_xpbd_2d::components::LinearVelocity;

use cursor_hero_character::character_plugin::Character;

pub struct LevelBoundsPlugin;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum LevelBoundsSystemSet {
    Spawn,
    Enforce,
}

impl Plugin for LevelBoundsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LevelBounds>()
            .configure_sets(Startup, LevelBoundsSystemSet::Spawn)
            .configure_sets(Update, LevelBoundsSystemSet::Enforce)
            .add_systems(Startup, spawn_parent.in_set(LevelBoundsSystemSet::Spawn))
            .add_systems(Update, enforce.in_set(LevelBoundsSystemSet::Enforce));
    }
}

#[derive(Component, Reflect)]
pub struct LevelBoundsParent;
#[derive(Component, Reflect)]
pub struct LevelBounds;

fn spawn_parent(mut commands: Commands) {
    info!("Spawning level bounds");
    commands.spawn((
        SpatialBundle::default(),
        LevelBoundsParent,
        Name::new("Level Bounds"),
    ));
}

#[allow(clippy::type_complexity)]
fn enforce(
    mut character_query: Query<
        (Entity, &Transform, &mut LinearVelocity),
        (With<Character>, Without<LevelBounds>),
    >,
    level_bounds: Query<
        (&Transform, &Sprite, &CollidingEntities),
        (With<LevelBounds>, Without<Character>),
    >,
) {
    for (character_entity, character_transform, mut character_velocity) in
        character_query.iter_mut()
    {
        let mut is_in_bounds = false;
        for bounds in level_bounds.iter() {
            if bounds.2 .0.contains(&character_entity) {
                is_in_bounds = true;
                break;
            }
        }
        if !is_in_bounds {
            // apply a force to to the character in the direction of the nearest boundary
            let mut nearest_boundary = None;
            let mut nearest_boundary_distance = f32::MAX;
            for bounds in level_bounds.iter() {
                let distance = character_transform
                    .translation
                    .distance(bounds.0.translation);
                if distance < nearest_boundary_distance {
                    nearest_boundary_distance = distance;
                    nearest_boundary = Some(bounds.0.translation);
                }
            }
            if let Some(nearest_boundary) = nearest_boundary {
                let direction = nearest_boundary - character_transform.translation;
                character_velocity.0 +=
                    direction.normalize().xy() * direction.length_squared() / 1000.0;
            }
        }
    }
}
