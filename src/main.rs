use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod screen_plugin;
use camera_plugin::CameraPlugin;
use capture_methods::screenlib_plugin::ScreenLibCapturePlugin;
use click_drag_movement_plugin::ClickDragMovementPlugin;
use position_text_plugin::PositionTextPlugin;
use hovershower_button_plugin::HoverShowerButtonPlugin;
use screen_plugin::ScreenPlugin;

mod character_plugin;
use character_plugin::CharacterPlugin;

mod camera_plugin;
mod capture_methods;
mod position_text_plugin;
mod hovershower_button_plugin;
mod metrics;
mod click_drag_movement_plugin;

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
            // Adds frame time diagnostics
            FrameTimeDiagnosticsPlugin,
            // Adds a system that prints diagnostics to the console
            LogDiagnosticsPlugin::default(),
        ))
        .add_plugins((
            ScreenPlugin,
            CharacterPlugin,
            InhouseCapturePlugin,
            InhouseThreadedCapturePlugin,
            ScreenLibCapturePlugin,
            CameraPlugin,
            HoverShowerButtonPlugin,
            PositionTextPlugin,
            ClickDragMovementPlugin
        ))
        .run();
}
