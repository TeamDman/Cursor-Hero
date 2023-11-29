/// Test to ensure that an object at rest is still considered colliding with sensors

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_xpbd_2d::{math::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cursor Hero Example - Zero Velocity Colliding".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vector::ZERO))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .add_systems(Startup, setup)
        .add_event::<MovementAction>()
        .add_systems(
            Update,
            (
                keyboard_input,
                apply_deferred,
                movement,
                apply_movement_damping,
                apply_pressure_plate_colour,
                update_velocity_text,
                log_events,
            )
                .chain(),
        )
        .run();
}

#[derive(Component, Default, Reflect)]
struct Character;

#[derive(Component, Default, Reflect)]
struct PressurePlate;

#[derive(Component, Default, Reflect)]
struct CharacterVelocityText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Capsule {
                        radius: 12.5,
                        depth: 20.0,
                        ..default()
                    }
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.2, 0.7, 0.9))),
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..default()
        },
        Character,
        RigidBody::Dynamic,
        Collider::capsule(20.0, 12.5),
        Name::new("Character")
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 150.0, 0.0),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        },
        PressurePlate,
        Sensor,
        RigidBody::Static,
        Collider::cuboid(100.0, 100.0),
        Name::new("Pressure Plate"),
    ));

    commands.spawn((
        TextBundle::from_section(
            "Velocity: ",
            TextStyle {
                font_size: 16.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        CharacterVelocityText,
        Name::new("Character Velocity Text"),
    ));

    commands.spawn(Camera2dBundle::default());
}

#[derive(Event, Debug, Reflect)]
pub enum MovementAction {
    Move(Vec2),
}

fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);
    let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vec2::new(horizontal as Scalar, vertical as Scalar);
    if direction != Vec2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction));
    }
}

fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<&mut LinearVelocity, With<Character>>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();
    let movement_acceleration = 2000.0;
    for event in movement_event_reader.read() {
        for mut linear_velocity in &mut controllers {
            match event {
                MovementAction::Move(direction) => {
                    linear_velocity.x += direction.x * movement_acceleration * delta_time;
                    linear_velocity.y += direction.y * movement_acceleration * delta_time;
                }
            }
        }
    }
}

fn apply_movement_damping(mut query: Query<(&mut LinearVelocity, &mut AngularVelocity)>) {
    let damping_factor = 0.95;
    for (mut linear_velocity, mut angular_velocity) in &mut query {
        linear_velocity.x *= damping_factor;
        linear_velocity.y *= damping_factor;
        angular_velocity.0 *= damping_factor;
    }
}


fn apply_pressure_plate_colour(
    mut query: Query<(&mut Sprite, &CollidingEntities), With<PressurePlate>>,
) {
    for (mut sprite, colliding_entities) in &mut query {
        if colliding_entities.0.is_empty() {
            sprite.color = Color::rgb(0.2, 0.7, 0.9);
        } else {
            sprite.color = Color::rgb(0.9, 0.7, 0.2);
        }
    }
}

fn update_velocity_text(
    character_query: Query<&LinearVelocity, With<Character>>,
    mut text_query: Query<&mut Text, With<CharacterVelocityText>>,
) {
    if let Ok(velocity) = character_query.get_single() {
        text_query.single_mut().sections[0].value = format!(
            "Velocity: {}, {}",
            velocity.x, velocity.y
        );
    }
}

fn log_events(
    mut started: EventReader<CollisionStarted>,
    mut ended: EventReader<CollisionEnded>,
) {
    // print out the started and ended events
    for event in started.read() {
        println!("CollisionStarted: {:?}", event);
    }
    for event in ended.read() {
        println!("CollisionEnded: {:?}", event);
    }
}