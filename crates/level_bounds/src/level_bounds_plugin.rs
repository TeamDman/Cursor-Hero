use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::CollidingEntities;
use bevy_xpbd_2d::components::LinearVelocity;

use bevy_xpbd_2d::components::RigidBody;
use bevy_xpbd_2d::components::Sensor;
use cursor_hero_character_types::prelude::*;
use cursor_hero_environment_types::prelude::*;

pub struct LevelBoundsPlugin;

impl Plugin for LevelBoundsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LevelBounds>();
        app.register_type::<LevelBoundsHolder>();
        app.register_type::<LevelBoundsParentRef>();
        app.add_event::<LevelBoundsEvent>();
        app.add_systems(Update, enforce);
        app.add_systems(
            Update,
            (
                handle_populate_environment_events,
                apply_deferred,
                handle_level_bounds_events,
            )
                .chain(),
        );
    }
}

#[derive(Event, Reflect, Debug, Clone, Copy)]
pub enum LevelBoundsEvent {
    AddPlayArea { environment_id: Entity, area: Rect },
}

#[derive(Component, Reflect)]
pub struct LevelBoundsHolder;
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
    environment_query: Query<(), Or<(With<HostEnvironment>, With<GameEnvironment>)>>,
) {
    for event in events.read() {
        if !environment_query.contains(event.environment_id) {
            continue;
        }

        info!(
            "Populating environment {:?} with level bounds parent",
            event
        );
        let mut level_bounds_holder_id = None;
        commands.entity(event.environment_id).with_children(|parent| {
            level_bounds_holder_id = Some(
                parent
                    .spawn((
                        SpatialBundle::default(),
                        LevelBoundsHolder,
                        Name::new("Level Bounds"),
                    ))
                    .id(),
            );
        });
        let Some(level_bounds_holder_id) = level_bounds_holder_id else {
            warn!(
                "Failed to create level bounds holder for environment {:?}",
                event
            );
            continue;
        };
        commands
            .entity(event.environment_id)
            .insert(LevelBoundsParentRef(level_bounds_holder_id));
    }
}

pub fn handle_level_bounds_events(
    mut events: EventReader<LevelBoundsEvent>,
    environment_query: Query<&LevelBoundsParentRef, With<EnvironmentKind>>,
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
                if let Ok(level_bounds_parent_ref) = environment_query.get(*environment_id) {
                    info!(
                        "Adding play area with size {:?} to level bounds for environment {:?}",
                        area.size(),
                        environment_id
                    );
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
                    new_deferred.push(*event);
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
