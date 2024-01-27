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

#[derive(Event, Reflect, Debug)]
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
                info!("Spawning level bounds parent for {:?}", event);
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
                if let Some(level_bounds_holder_id) = level_bounds_holder_id {
                    commands
                        .entity(*environment_id)
                        .insert(LevelBoundsParentRef(level_bounds_holder_id));
                } else {
                    unreachable!("Level bounds parent should exist by now");
                }
            }
        }
    }
}

pub fn handle_level_bounds_events(
    mut events: EventReader<LevelBoundsEvent>,
    environment_query: Query<(&Name, &LevelBoundsParentRef), With<Environment>>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event {
            LevelBoundsEvent::AddPlayArea {
                environment_id,
                area,
            } => {
                info!(
                    "Adding play area {:?} ({:?}) to level bounds for environment {:?}",
                    area,
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
                    unreachable!("Level bounds parent should exist by now");
                }
            }
        }
    }
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
