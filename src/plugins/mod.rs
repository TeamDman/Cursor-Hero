pub mod active_input_state_plugin;
pub mod afterimage_plugin;
pub mod camera_plugin;
pub mod character_plugin;
pub mod click_drag_movement_plugin;
pub mod cursor_mirroring_plugin;
pub mod fps_text_plugin;
pub mod hover_shower_relay_plugin;
pub mod hover_shower_service_plugin;
pub mod hover_ui_automation_plugin;
pub mod position_text_plugin;
pub mod pressure_plate_plugin;
pub mod screen_plugin;
pub mod screen_update_plugin;
pub mod tools;

use bevy::prelude::*;

use self::{
    active_input_state_plugin::ActiveInputStatePlugin, afterimage_plugin::AfterimagePlugin,
    camera_plugin::CameraPlugin, character_plugin::CharacterPlugin,
    click_drag_movement_plugin::ClickDragMovementPlugin,
    cursor_mirroring_plugin::CursorMirroringPlugin, fps_text_plugin::FpsTextPlugin,
    hover_shower_relay_plugin::HoverShowerRelayPlugin,
    hover_shower_service_plugin::HoverShowerServicePlugin,
    hover_ui_automation_plugin::HoverUiAutomationPlugin, position_text_plugin::PositionTextPlugin,
    pressure_plate_plugin::PressurePlatePlugin, screen_plugin::ScreenPlugin,
    screen_update_plugin::ScreenUpdatePlugin, tools::ToolsPlugin,
};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ActiveInputStatePlugin,
            AfterimagePlugin,
            CameraPlugin,
            CharacterPlugin,
            ClickDragMovementPlugin,
            FpsTextPlugin,
            // HoverShowerRelayPlugin,
            // HoverShowerServicePlugin,
            PositionTextPlugin,
            PressurePlatePlugin,
            ScreenPlugin,
            ScreenUpdatePlugin,
        ))
        .add_plugins((CursorMirroringPlugin, HoverUiAutomationPlugin, ToolsPlugin));
    }
}
