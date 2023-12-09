use std::time::Duration;

use bevy::{prelude::*, utils::Instant};
use bevy_xpbd_2d::components::{Collider, RigidBody, Sensor};
use crossbeam_channel::{bounded, Receiver};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::pressure_plate_plugin::{
    PressurePlate, PressurePlateActivationEvent, PressurePlateProgressIndicator,
};
pub struct ProcessCommunicationPlugin;
impl Plugin for ProcessCommunicationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HoverShowerPressurePlate>()
            .register_type::<MyDebugText>()
            .add_event::<StreamEvent>()
            .add_systems(Startup, setup)
            // .add_systems(
            //     Update,
            //     (handle_activation),
            // );
            ;
    }
}

#[derive(Component, Reflect)]
pub struct HoverShowerPressurePlate;

#[derive(Component, Reflect)]
pub struct MyDebugText;

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<u32>);

#[derive(Event)]
struct StreamEvent(u32);

fn setup(mut commands: Commands) {
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
            Name::new("SpawnProcessPressurePlate Indicator"),
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
        Name::new("SpawnProcessPressurePlate"),
    ));

    
    // spawn the listener thread

    let (tx, rx) = bounded::<u32>(10);
    std::thread::spawn(move || {
        let mut rng = StdRng::seed_from_u64(19878367467713);
        loop {
            // sleep for 1 second
            std::thread::sleep(Duration::from_secs(1));

            tx.send(rng.gen_range(0..2000)).unwrap();
        }
    });

    commands.insert_resource(StreamReceiver(rx));
}

// fn handle_activation(
//     mut commands: Commands,
//     mut reader: EventReader<PressurePlateActivationEvent>,
//     query: Query<&HoverShowerPressurePlate>,
// ) {
//     for event in reader.read() {
//         if let Ok(plate) = query.get(event.0) {
//             // spawn the dotnet process
//             let mut command = std::process::Command::new("wt");
//             command.args(
//                 r"pwsh -NoProfile -c D:\Repos\Games\Cursor-Hero\other\start-hovershower.ps1"
//                     .split(' ')
//                     .map(|s| s.to_string())
//                     .collect(),
//             );
//             command.spawn().expect("failed to spawn process");
//         }
//     }
// }

// // This system reads from the receiver and sends events to Bevy
// fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
//     for from_stream in receiver.try_iter() {
//         events.send(StreamEvent(from_stream));
//     }
// }

// fn spawn_text(mut commands: Commands, mut reader: EventReader<StreamEvent>) {
//     let text_style = TextStyle {
//         font_size: 20.0,
//         color: Color::WHITE,
//         ..default()
//     };

//     for (per_frame, event) in reader.read().enumerate() {
//         commands.spawn((
//             Text2dBundle {
//                 text: Text::from_section(event.0.to_string(), text_style.clone())
//                     .with_alignment(TextAlignment::Center),
//                 transform: Transform::from_xyz(per_frame as f32 * 100.0, 300.0, 0.0),
//                 ..default()
//             },
//             MyDebugText,
//         ));
//     }
// }

// fn move_text(
//     mut commands: Commands,
//     mut texts: Query<(Entity, &mut Transform), With<MyDebugText>>,
//     time: Res<Time>,
// ) {
//     for (entity, mut position) in &mut texts {
//         position.translation -= Vec3::new(0.0, 100.0 * time.delta_seconds(), 0.0);
//         if position.translation.y < -300.0 {
//             commands.entity(entity).despawn();
//         }
//     }
// }
