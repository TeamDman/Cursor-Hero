use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod screen_plugin;
use camera_plugin::CameraPlugin;
use capture_methods::screenlib_plugin::ScreenLibCapturePlugin;
use character_position_plugin::CharacterPositionPlugin;
use hovershower_button_plugin::HoverShowerButtonPlugin;
use screen_plugin::ScreenPlugin;

mod character_plugin;
use character_plugin::CharacterPlugin;

mod capture_methods;
mod hovershower_button_plugin;
mod metrics;
mod character_position_plugin;
mod camera_plugin;

use crate::capture_methods::inhouse_plugin::InhouseCapturePlugin;
use crate::capture_methods::inhouse_threaded_plugin::InhouseThreadedCapturePlugin;

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
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        )
        .add_plugins((
            ScreenPlugin, // load before character plugin to avoid stuttering 🤷‍♂️
            CharacterPlugin,
            InhouseCapturePlugin,
            InhouseThreadedCapturePlugin,
            ScreenLibCapturePlugin,
            CameraPlugin,
            HoverShowerButtonPlugin,
            CharacterPositionPlugin,
        ))
        .run();
}
