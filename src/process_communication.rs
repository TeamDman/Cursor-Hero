use std::{fmt::Formatter, time::Duration};

use bevy::prelude::*;
use bevy_xpbd_2d::components::{Collider, RigidBody, Sensor};
use crossbeam_channel::{bounded, Receiver};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Deserialize;
use std::fmt::Display;
use std::thread;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::windows::named_pipe::ClientOptions;

use crate::pressure_plate_plugin::{
    PressurePlate, PressurePlateActivationEvent, PressurePlateProgressIndicator,
};
pub struct HoverShowerRelayPlugin;
impl Plugin for HoverShowerRelayPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HoverShowerPressurePlate>()
            .register_type::<MyDebugText>()
            .register_type::<Afterimage>()
            .register_type::<TextParent>()
            .add_event::<StreamEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    handle_activation,
                    read_stream,
                    spawn_text,
                    move_text,
                    spawn_hints,
                    tick_afterimages,
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

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<ReceivedData>);

#[derive(Event)]
struct StreamEvent(ReceivedData);

#[derive(Debug, Deserialize)]
struct ElementDetails {
    name: String,
    boundingRect: Vec<i32>,
    controlType: String,
    className: String,
    automationId: String,
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct InterestingElement {
    details: ElementDetails,
    depth: i32,
    relationship: String,
}

#[derive(Component, Debug, Reflect)]
struct Afterimage {
    pub life_remaining: usize,
}
impl Default for Afterimage {
    fn default() -> Self {
        Self {
            life_remaining: 100,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ReceivedData {
    cursorPosition: Vec<i32>,
    elementDetails: ElementDetails,
    interestingElements: Vec<InterestingElement>,
}

impl Display for ReceivedData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReceivedData {{ cursorPosition: {:?}, elementDetails: {:?}, interestingElements: {:?} }}", self.cursorPosition, self.elementDetails, self.interestingElements)
    }
}

fn setup(mut commands: Commands) {
    let mut parent = commands.spawn((
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
        HoverShowerPressurePlate,
        Sensor,
        RigidBody::Static,
        Collider::cuboid(100.0, 100.0), // make the interaction range larger than the button itself
        Name::new("Hover Shower Pressure Plate Activator"),
    ));

    // spawn the listener thread

    let (tx, rx) = bounded::<ReceivedData>(10);
    commands.insert_resource(StreamReceiver(rx));

    // let mut rng = StdRng::seed_from_u64(19878367467713);
    const ERROR_PIPE_BUSY: i32 = 231;
    const PIPE_NAME: &str = r"\\.\pipe\hovershower";

    // Tokio thread
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            'new_connection: loop {
                let client = loop {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    match ClientOptions::new().open(PIPE_NAME) {
                        Ok(client) => break client,
                        Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) => (),
                        Err(e) => {
                            info!("Error opening client: {}", e);
                            continue;
                        }
                    }
                };

                let mut reader = BufReader::new(client); // Pass ownership
                let mut line = String::new();

                // Reading the incoming Fibonacci numbers
                loop {
                    reader
                        .read_line(&mut line)
                        .await
                        .expect("Couldn't read line");

                    let received_data: Result<ReceivedData, serde_json::Error> =
                        serde_json::from_str(&line);
                    match received_data {
                        Ok(recv) => {
                            debug!("Cursor position: {:?}", recv.cursorPosition);
                            tx.send(recv).unwrap();
                        }
                        Err(e) => {
                            warn!("Couldn't deserialize data: {}", e);
                            if line == "" {
                                println!("Pipe closed");
                                continue 'new_connection;
                            }
                        }
                    }

                    line.clear();
                }
            }
        });
    });
}

fn handle_activation(
    mut commands: Commands,
    mut reader: EventReader<PressurePlateActivationEvent>,
    query: Query<&HoverShowerPressurePlate>,
) {
    for event in reader.read() {
        if let Ok(plate) = query.get(event.0) {
            // spawn the dotnet process
            let mut command = std::process::Command::new("wt");
            command.args(vec![
                "pwsh",
                "-NoProfile",
                "-c",
                r"D:\Repos\Games\Cursor-Hero\other\start-hovershower.ps1",
            ]);
            command.spawn().expect("failed to spawn process");
        }
    }
}

// This system reads from the receiver and sends events to Bevy
fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
    for from_stream in receiver.try_iter() {
        events.send(StreamEvent(from_stream));
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
        for (per_frame, event) in reader.read().enumerate() {
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

fn spawn_hints(
    mut commands: Commands,
    mut reader: EventReader<StreamEvent>,
    asset_server: Res<AssetServer>,
    mut count: Local<usize>,
) {
    for event in reader.read() {
        // spawn cursor
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    event.0.cursorPosition[0] as f32,
                    -event.0.cursorPosition[1] as f32,
                    20.0,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                texture: asset_server.load("character.png"),
                ..default()
            },
            Afterimage::default(),
            Name::new(format!("Cursor {}", count.clone())),
        ));

        let details = &event.0.elementDetails;
        // spawn element
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    details.boundingRect[0] as f32 + details.boundingRect[2] as f32 / 2.0,
                    -details.boundingRect[1] as f32 - details.boundingRect[3] as f32 / 2.0,
                    20.0,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        details.boundingRect[2] as f32,
                        details.boundingRect[3] as f32,
                    )),
                    color: Color::rgba(0.141, 0.675, 0.949, 0.05),
                    ..default()
                },
                ..default()
            },
            Afterimage::default(),
            Name::new(format!("Element {}", count.clone())),
        ));
    }
}

fn tick_afterimages(mut commands: Commands, mut afterimages: Query<(Entity, &mut Afterimage)>) {
    for (entity, mut afterimage) in &mut afterimages.iter_mut() {
        afterimage.life_remaining -= 1;
        if afterimage.life_remaining == 0 {
            commands.entity(entity).despawn();
        }
    }
}
