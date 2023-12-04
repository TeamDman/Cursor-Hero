use bevy::prelude::*;
use bevy_xpbd_2d::components::{Collider, CollidingEntities, RigidBody, Sensor};

/// This vertex attribute supplies barycentric coordinates for each triangle.
/// Each component of the vector corresponds to one corner of a triangle. It's
/// equal to 1.0 in that corner and 0.0 in the other two. Hence, its value in
/// the fragment shader indicates proximity to a corner or the opposite edge.
// const ATTRIBUTE_BARYCENTRIC: MeshVertexAttribute =
//     MeshVertexAttribute::new("Barycentric", 2137464976, VertexFormat::Float32x3);

pub struct HoverShowerButtonPlugin;
impl Plugin for HoverShowerButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_button)
            .add_systems(Update, update_plate)
            .register_type::<PressurePlate>();
    }
}

#[derive(Component, Reflect)]
struct PressurePlate {
    active_time: f32,
    debounce: bool,
    indicator: Entity,
}

impl PressurePlate {
    fn new(indicator: Entity) -> Self {
        Self {
            active_time: 0.0,
            debounce: false,
            indicator,
        }
    }
}

#[derive(Component, Default, Reflect)]
struct PressurePlateProgressIndicator {
    visual_progress: f32,
}

fn spawn_button(mut commands: Commands) {
    let indicator = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(10.0, 160.0, 15.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(80.0, 80.0)),
                    ..default()
                },
                ..default()
            },
            PressurePlateProgressIndicator::default(),
            Name::new("HoverShower Button"),
        ))
        .id();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 150.0, 10.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        },
        PressurePlate::new(indicator),
        Sensor,
        RigidBody::Static,
        Collider::cuboid(100.0, 100.0), // make the interaction range larger than the button itself
        Name::new("HoverShower Button"),
    ));

    // A cube to move around
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.4, 0.7),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(250.0, -100.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(30.0, 30.0),
        Name::new("A cube to push around"),
    ));
}

fn update_plate(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut PressurePlate,
            &mut Sprite,
            &CollidingEntities,
            Option<&SpatialAudioSink>,
        ),
        Without<PressurePlateProgressIndicator>,
    >,
    mut indicator_query: Query<(&mut PressurePlateProgressIndicator, &mut Sprite), Without<PressurePlate>>,
) {
    for (entity, mut plate, mut sprite, colliding_entities, sink) in &mut query {
        if colliding_entities.0.is_empty() {
            sprite.color = Color::rgb(0.2, 0.7, 0.9);
            plate.active_time = 0.0;
            sink.map(SpatialAudioSink::stop);
            plate.debounce = false;
        } else {
            if plate.debounce {
                continue;
            }
            sprite.color = Color::rgb(0.9, 0.7, 0.2);
            if plate.active_time == 0.0 {
                let bundle = AudioBundle {
                    source: asset_server.load("sounds/pressure plate activation.ogg"),
                    settings: PlaybackSettings::REMOVE.with_spatial(true),
                    ..default()
                };
                commands.entity(entity).insert(bundle);
                plate.active_time += time.delta_seconds();
            } else {
                plate.active_time += time.delta_seconds();
                if plate.active_time > crate::sounds::PRESSURE_PLATE_ACTIVATION_DURATION {
                    plate.active_time = 0.0;
                    println!("Activated!");
                    plate.debounce = true;
                }
            }
        }
        if let Ok((mut indicator, mut indicator_sprite)) = indicator_query.get_mut(plate.indicator)
        {
            indicator.visual_progress =
                plate.active_time / crate::sounds::PRESSURE_PLATE_ACTIVATION_DURATION;
            indicator_sprite.color = Color::rgb(0.2, 0.7, 0.9) * indicator.visual_progress;
        }
    }
}
