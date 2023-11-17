use bevy::{prelude::*, input::mouse::MouseWheel};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::input::common_conditions::input_toggle_active;

mod screen_backgrounds;
use screen_backgrounds::ScreenBackgroundsPlugin;

mod cursor_character;
use cursor_character::{CursorCharacterPlugin, Character};

mod windows_screen_capturing;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cursor Hero".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)))
        .add_plugins((ScreenBackgroundsPlugin, CursorCharacterPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_follow_tick, camera_zoom_tick))
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn camera_zoom_tick(
    mut cam: Query<&mut Transform, With<Camera>>,
    mut scroll: EventReader<MouseWheel>,
) {
    for event in scroll.iter() {
        cam.single_mut().scale *= Vec3::splat(1.0 - event.y / 10.0);
    }
}

fn camera_follow_tick(
    mut cam: Query<&mut Transform, With<Camera>>,
    char: Query<(&Transform, (With<Character>, Without<Camera>))>,
) {
    cam.single_mut().translation = char.single().0.translation;
}