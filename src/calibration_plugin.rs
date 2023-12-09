use bevy::prelude::*;
use bevy_xpbd_2d::components::{Collider, RigidBody, Sensor};
use crate::{pressure_plate_plugin::{
    PressurePlate, PressurePlateActivationEvent, PressurePlateProgressIndicator,
}, hovershower_service_plugin::{ReceivedData, StreamEvent}};
pub struct CalibrationPlugin;
impl Plugin for CalibrationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
        .register_type::<CalibrationParent>()
        .register_type::<CalibrationPressurePlate>()
        .register_type::<CalibrationData>()
        .add_systems(
            Update,
            (handle_activation, track_mouse),
        );
    }
}

// structs
#[derive(Component, Reflect)]
pub struct CalibrationParent;

#[derive(Component, Reflect)]
pub struct CalibrationPressurePlate;

#[derive(Component, Reflect)]
pub struct CalibrationData {
    mouse_history: Vec<ReceivedData>,
}

// systems
fn setup(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        CalibrationPressurePlate,
        Name::new("Calibration Pressure Plate"),
    ));
    let mut parent = commands.spawn((
        SpatialBundle::default(),
        CalibrationParent,
        Name::new("Calibration Parent"),
    ));
    parent.with_children(|parent| {
        let indicator = parent
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(110.0, 160.0, 15.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(80.0, 80.0)),
                        ..default()
                    },
                    ..default()
                },
                PressurePlateProgressIndicator::default(),
                Name::new("Calibration Pressure Plate Indicator"),
            ))
            .id();
        let _activator = parent.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(100.0, 150.0, 10.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                ..default()
            },
            PressurePlate::new(indicator),
            CalibrationPressurePlate,
            Sensor,
            RigidBody::Static,
            Collider::cuboid(100.0, 100.0), // make the interaction range larger than the button itself
            Name::new("Calibration Pressure Plate Activator"),
        ));
    });
}

fn handle_activation(
    mut commands: Commands,
    mut reader: EventReader<PressurePlateActivationEvent>,
    query: Query<&CalibrationPressurePlate>,
) {
    for event in reader.read() {
        if let Ok(_plate) = query.get(event.0) {
            info!("Begin calibration");
            commands.spawn((CalibrationData {
                mouse_history: Vec::new(),
            },));
        }
    }
}

fn track_mouse(
    // mut commands: Commands,
    mut reader: EventReader<StreamEvent>,
    mut query: Query<&mut CalibrationData>,
) {
    for event in reader.read() {
        for mut data in query.iter_mut() {
            data.mouse_history.push(event.0.clone());
            calculate_screen_to_world(&data.mouse_history);
        }
    }
}

fn calculate_screen_to_world(mouse_history: &Vec<ReceivedData>) {
    mouse_history.iter().for_each(|data| {
        let cursor_position = &data.cursor_position;
        let element_details = &data.element_details;
        let interesting_elements = &data.interesting_elements;
        debug!("Cursor position: {:?}", cursor_position);
        debug!("Element details: {:?}", element_details);
        debug!("Interesting elements: {:?}", interesting_elements);
    });
}
