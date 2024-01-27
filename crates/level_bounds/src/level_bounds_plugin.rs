use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::CollidingEntities;
use bevy_xpbd_2d::components::LinearVelocity;

use bevy_xpbd_2d::components::RigidBody;
use bevy_xpbd_2d::components::Sensor;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_environment::environment_plugin::Environment;
use cursor_hero_environment::environment_plugin::PopulateEnvironmentEvent;

pub struct LevelBoundsPlugin;

impl Plugin for LevelBoundsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LevelBounds>()
            .register_type::<LevelBoundsParent>()
            .add_event::<LevelBoundsEvent>()
            .add_systems(
                Update,
                (
                    (
                        handle_populate_environment_events,
                        apply_deferred,
                        handle_level_bounds_events,
                    )
                        .chain(),
                    enforce,
                ),
            );
    }
}

#[derive(Event, Reflect, Debug, Clone, Copy)]
pub enum LevelBoundsEvent {
    AddPlayArea { environment_id: Entity, area: Rect },
}

#[derive(Component, Reflect)]
pub struct LevelBoundsParent;
#[derive(Component, Reflect)]
pub struct LevelBoundsParentRef(Entity);
impl LevelBoundsParentRef {
    pub fn get(&self) -> Entity {
        self.0
    }
}
#[derive(Component, Reflect)]
pub struct LevelBounds;

fn handle_populate_environment_events(
    mut commands: Commands,
    mut events: EventReader<PopulateEnvironmentEvent>,
) {
    for event in events.read() {
        match event {
            PopulateEnvironmentEvent::Host { environment_id }
            | PopulateEnvironmentEvent::Game { environment_id } => {
                info!(
                    "Populating environment {:?} with level bounds parent",
                    event
                );
                let mut level_bounds_holder_id = None;
                commands.entity(*environment_id).with_children(|parent| {
                    level_bounds_holder_id = Some(
                        parent
                            .spawn((
                                SpatialBundle::default(),
                                LevelBoundsParent,
                                Name::new("Level Bounds"),
                            ))
                            .id(),
                    );
                });
                commands
                    .entity(*environment_id)
                    .insert(LevelBoundsParentRef(
                        level_bounds_holder_id.expect("we just created this entity"),
                    ));
            }
        }
    }
}

pub fn handle_level_bounds_events(
    mut events: EventReader<LevelBoundsEvent>,
    environment_query: Query<(&Name, &LevelBoundsParentRef), With<Environment>>,
    mut commands: Commands,
    mut deferred: Local<Vec<LevelBoundsEvent>>,
) {
    let mut new_deferred = Vec::new();
    for event in events.read().chain(deferred.into_iter()) {
        match event {
            LevelBoundsEvent::AddPlayArea {
                environment_id,
                area,
            } => {
                info!(
                    "Adding play area with size {:?} to level bounds for environment {:?}",
                    area.size(),
                    environment_id
                );
                if let Ok((environment_name, level_bounds_parent_ref)) =
                    environment_query.get(*environment_id)
                {
                    commands
                        .entity(level_bounds_parent_ref.get())
                        .with_children(|parent| {
                            parent.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(area.size()),
                                        color: Color::ORANGE,
                                        ..default()
                                    },
                                    transform: Transform::from_translation(
                                        area.center().extend(-2.0),
                                    ),
                                    visibility: Visibility::Hidden,
                                    ..default()
                                },
                                Sensor,
                                RigidBody::Static,
                                Collider::cuboid(area.size().x, area.size().y),
                                LevelBounds,
                                Name::new("Level Bounds"),
                            ));
                        });
                } else {
                    debug!(
                        "Deferring level bounds event for environment {:?}",
                        environment_id
                    );
                    new_deferred.push(event.clone());
                }
            }
        }
    }
    *deferred = new_deferred;
}

#[allow(clippy::type_complexity)]
fn enforce(
    mut character_query: Query<
        (Entity, &GlobalTransform, &mut LinearVelocity),
        (With<Character>, Without<LevelBounds>),
    >,
    level_bounds: Query<
        (&GlobalTransform, &CollidingEntities),
        (With<LevelBounds>, Without<Character>),
    >,
) {
    for (character_entity, character_transform, mut character_velocity) in
        character_query.iter_mut()
    {
        let mut is_in_bounds = false;
        for (_, in_bounds) in level_bounds.iter() {
            if in_bounds.contains(&character_entity) {
                is_in_bounds = true;
                break;
            }
        }
        if !is_in_bounds {
            // debug!("Found {} level bounds", level_bounds.iter().count());
            // apply a force to to the character in the direction of the nearest boundary
            let mut nearest_boundary = None;
            let mut nearest_boundary_distance = f32::MAX;
            for (bounds_transform, _) in level_bounds.iter() {
                let distance = character_transform
                    .translation()
                    .distance(bounds_transform.translation());
                // debug!("Distance to boundary: {}", distance);
                if distance < nearest_boundary_distance {
                    nearest_boundary_distance = distance;
                    nearest_boundary = Some(bounds_transform.translation());
                }
            }
            if let Some(nearest_boundary) = nearest_boundary {
                let direction = nearest_boundary - character_transform.translation();
                character_velocity.0 +=
                    direction.normalize().xy() * direction.length_squared() / 1000.0;
            }
        }
    }
}
