// mostly from 2d kinematic character example from https://github.com/Jondolf/bevy_xpbd

use bevy::input::common_conditions::input_toggle_active;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::{math::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cursor Hero Example - Jitter".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        )
        .insert_resource(Gravity(Vector::ZERO))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .add_systems(Startup, setup)
        .add_systems(Update, camera_follow_update.after(PhysicsSet::Sync))
        .run();
}

#[derive(Component, Reflect)]
pub struct Character;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut velocity = LinearVelocity::default();
    velocity.x = 5000.0;
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
        RigidBody::Dynamic,
        Character,
        Collider::capsule(20.0, 12.5),
        velocity,
    ));

    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn camera_follow_update(
    mut cam: Query<&mut Transform, (With<Camera>, Without<Character>)>,
    character: Query<&Transform, (With<Character>, Without<Camera>)>,
) {
    let mut cam_pos = cam.single_mut();
    let character_pos = character.single();
    cam_pos.translation.x = character_pos.translation.x;
    cam_pos.translation.y = character_pos.translation.y;
}
