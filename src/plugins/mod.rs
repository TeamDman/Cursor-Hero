pub mod active_input_state_plugin;
pub mod afterimage_plugin;
pub mod camera_plugin;
pub mod character_plugin;
pub mod click_drag_movement_plugin;
pub mod cursor_mirroring_plugin;
pub mod damping_plugin;
pub mod fps_text_plugin;
pub mod hover_shower_relay_plugin;
pub mod hover_shower_service_plugin;
pub mod hover_ui_automation_plugin;
pub mod pointer_plugin;
pub mod position_text_plugin;
pub mod pressure_plate_plugin;
pub mod screen_plugin;
pub mod screen_update_plugin;
pub mod toolbelt;
pub mod tools;

use bevy::prelude::*;

use self::{
    active_input_state_plugin::ActiveInputStatePlugin,
    afterimage_plugin::AfterimagePlugin,
    camera_plugin::CameraPlugin,
    character_plugin::CharacterPlugin,
    click_drag_movement_plugin::ClickDragMovementPlugin,
    cursor_mirroring_plugin::CursorMirroringPlugin,
    damping_plugin::DampingPlugin,
    fps_text_plugin::FpsTextPlugin,
    // hover_shower_relay_plugin::HoverShowerRelayPlugin,
    // hover_shower_service_plugin::HoverShowerServicePlugin,
    hover_ui_automation_plugin::HoverUiAutomationPlugin,
    pointer_plugin::PointerPlugin,
    position_text_plugin::PositionTextPlugin,
    pressure_plate_plugin::PressurePlatePlugin,
    screen_plugin::ScreenPlugin,
    screen_update_plugin::ScreenUpdatePlugin,
    toolbelt::toolbelt_plugin::ToolbeltPlugin,
    tools::tools_plugin::ToolsPlugin,
};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActiveInputStatePlugin)
            .add_plugins(AfterimagePlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(CharacterPlugin)
            .add_plugins(ClickDragMovementPlugin)
            .add_plugins(FpsTextPlugin)
            // .add_plugins(HoverShowerRelayPlugin)
            // .add_plugins(HoverShowerServicePlugin)
            .add_plugins(PositionTextPlugin)
            .add_plugins(PressurePlatePlugin)
            .add_plugins(ScreenPlugin)
            .add_plugins(ScreenUpdatePlugin)
            .add_plugins(CursorMirroringPlugin)
            .add_plugins(HoverUiAutomationPlugin)
            .add_plugins(ToolsPlugin)
            .add_plugins(ToolbeltPlugin)
            .add_plugins(PointerPlugin)
            .add_plugins(DampingPlugin);
    }
}
