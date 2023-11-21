use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::character_plugin::Character;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (camera_follow_tick, camera_zoom_tick));
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn camera_zoom_tick(
    mut cam: Query<&mut Transform, With<MainCamera>>,
    mut scroll: EventReader<MouseWheel>,
) {
    for event in scroll.read() {
        cam.single_mut().scale *= Vec3::splat(1.0 - event.y / 10.0);
    }
}

fn camera_follow_tick(
    mut cam: Query<&mut Transform, With<MainCamera>>,
    char: Query<(&Transform, (With<Character>, Without<MainCamera>))>, // we exclude the camera to guarantee queries are disjoint
) {
    cam.single_mut().translation = char.single().0.translation;
}
