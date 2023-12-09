use std::fmt::Formatter;

use bevy::prelude::*;
use bevy_xpbd_2d::components::{Collider, RigidBody, Sensor};
use std::fmt::Display;

use crate::afterimage_plugin::{Afterimage, AfterimageParent};
use crate::hovershower_service_plugin::{ReceivedData, StreamEvent, start_service_process};
use crate::pressure_plate_plugin::{
    PressurePlate, PressurePlateActivationEvent, PressurePlateProgressIndicator,
};
pub struct HoverShowerRelayPlugin;
impl Plugin for HoverShowerRelayPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HoverShowerPressurePlate>()
            .register_type::<MyDebugText>()
            .register_type::<TextParent>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    handle_activation,
                    spawn_text,
                    move_text,
                    spawn_afterimages,
                ),
            );
    }
}

#[derive(Component, Reflect)]
pub struct TextParent;

#[derive(Component, Reflect)]
pub struct HoverShowerPressurePlate;

#[derive(Component, Reflect)]
pub struct MyDebugText;

impl Display for ReceivedData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReceivedData {{ cursorPosition: {:?}, elementDetails: {:?}, interestingElements: {:?} }}", self.cursor_position, self.element_details, self.interesting_elements)
    }
}

fn setup(mut commands: Commands, mut writer: EventWriter<PressurePlateActivationEvent>) {
    commands.spawn((
        SpatialBundle::default(),
        TextParent,
        Name::new("HoverShower Text Parent"),
    ));

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
            Name::new("Hover Shower Pressure Plate Indicator"),
        ))
        .id();
    let activator = commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 150.0, 10.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        },
        PressurePlate::new(indicator),
        HoverShowerPressurePlate,
        Sensor,
        RigidBody::Static,
        Collider::cuboid(100.0, 100.0), // make the interaction range larger than the button itself
        Name::new("Hover Shower Pressure Plate Activator"),
    ));

    // immediately trigger the pressure plate
    writer.send(PressurePlateActivationEvent(activator.id()));
}

fn handle_activation(
    // mut commands: Commands,
    mut reader: EventReader<PressurePlateActivationEvent>,
    query: Query<&HoverShowerPressurePlate>,
) {
    for event in reader.read() {
        if let Ok(_plate) = query.get(event.0) {
            start_service_process().expect("failed to spawn process");
        }
    }
}

fn spawn_text(
    mut commands: Commands,
    parent: Query<Entity, With<TextParent>>,
    mut reader: EventReader<StreamEvent>,
    plate_pos: Query<&Transform, With<HoverShowerPressurePlate>>,
) {
    let text_style = TextStyle {
        font_size: 20.0,
        color: Color::WHITE,
        ..default()
    };
    commands.entity(parent.single()).with_children(|parent| {
        for (per_frame, _event) in reader.read().enumerate() {
            let mut text_pos = plate_pos.get_single().unwrap().clone();
            // Adjust the position directly
            text_pos.translation += Vec3::new(per_frame as f32 * 100.0, 300.0, 0.0);
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section("data", text_style.clone())
                        .with_alignment(TextAlignment::Center),
                    transform: text_pos,
                    ..default()
                },
                MyDebugText,
            ));
        }
    });
}

fn move_text(
    mut commands: Commands,
    mut texts: Query<(Entity, &mut Transform), With<MyDebugText>>,
    time: Res<Time>,
) {
    for (entity, mut position) in &mut texts {
        position.translation -= Vec3::new(0.0, 100.0 * time.delta_seconds(), 0.0);
        if position.translation.y < -300.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_afterimages(
    mut commands: Commands,
    mut reader: EventReader<StreamEvent>,
    asset_server: Res<AssetServer>,
    mut count: Local<usize>,
    parent: Query<Entity, With<AfterimageParent>>,
) {
    parent.for_each(|parent| {
        commands.entity(parent).with_children(|parent| {
            for event in reader.read() {
                // spawn cursor
                parent.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            event.0.cursor_position[0] as f32,
                            -event.0.cursor_position[1] as f32,
                            20.0,
                        ),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(20.0, 20.0)),
                            ..default()
                        },
                        texture: asset_server.load("character.png"),
                        ..default()
                    },
                    Afterimage { life_remaining: 50 },
                    Name::new(format!("Cursor {}", count.clone())),
                ));

                let details = &event.0.element_details;
                // spawn element
                parent.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            details.bounding_rect[0] as f32 + details.bounding_rect[2] as f32 / 2.0,
                            -details.bounding_rect[1] as f32 - details.bounding_rect[3] as f32 / 2.0,
                            20.0,
                        ),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(
                                details.bounding_rect[2] as f32,
                                details.bounding_rect[3] as f32,
                            )),
                            color: Color::rgba(0.141, 0.675, 0.949, 0.05),
                            ..default()
                        },
                        ..default()
                    },
                    Afterimage { life_remaining: 50 },
                    Name::new(format!("Element {}", count.clone())),
                ));

                *count += 1;
            }
        });
    })
}
