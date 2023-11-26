use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod screen_plugin;
use camera_plugin::CameraPlugin;
use capture_methods::screenlib_plugin::ScreenLibCapturePlugin;
use click_drag_movement_plugin::ClickDragMovementPlugin;
use fps_text_plugin::FpsTextPlugin;
use hovershower_button_plugin::HoverShowerButtonPlugin;
use interaction_plugin::InteractionPlugin;
use position_text_plugin::PositionTextPlugin;
use screen_plugin::ScreenPlugin;

mod character_plugin;
use character_plugin::CharacterPlugin;
use update_ordering::UpdateOrderingPlugin;

mod camera_plugin;
mod capture_methods;
mod click_drag_movement_plugin;
mod hovershower_button_plugin;
mod interaction_plugin;
mod metrics;
mod position_text_plugin;
mod update_ordering;
mod fps_text_plugin;

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
        .add_plugins((FrameTimeDiagnosticsPlugin,))
        .add_plugins((
            FpsTextPlugin,
            UpdateOrderingPlugin,
            ScreenPlugin,
            CharacterPlugin,
            InteractionPlugin,
            InhouseCapturePlugin,
            InhouseThreadedCapturePlugin,
            ScreenLibCapturePlugin,
            CameraPlugin,
            HoverShowerButtonPlugin,
            PositionTextPlugin,
            ClickDragMovementPlugin,
        ))
        .run();
}
