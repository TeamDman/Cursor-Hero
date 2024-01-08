use bevy::prelude::*;

use cursor_hero_camera::camera_plugin::CameraPlugin;
use cursor_hero_character::character_plugin::CharacterPlugin;
use cursor_hero_cursor_mirror::cursor_mirroring_plugin::CursorMirroringPlugin;
use cursor_hero_hover::afterimage_plugin::AfterimagePlugin;
use cursor_hero_hover::hover_ui_automation_plugin::HoverUiAutomationPlugin;
use cursor_hero_input::active_input_state_plugin::ActiveInputStatePlugin;
use cursor_hero_level_bounds::level_bounds_plugin::LevelBoundsPlugin;
// use cursor_hero_click_drag_character_movement::ClickDragMovementPlugin;
use cursor_hero_physics::damping_plugin::DampingPlugin;
use cursor_hero_pointer::pointer_plugin::PointerPlugin;
use cursor_hero_pressure_plate::pressure_plate_plugin::PressurePlatePlugin;
use cursor_hero_restart_memory::primary_window_memory_plugin::PrimaryWindowMemoryPlugin;
use cursor_hero_screen::screen_plugin::ScreenPlugin;
use cursor_hero_screen::screen_update_plugin::ScreenUpdatePlugin;
use cursor_hero_toolbelt::ToolbeltPlugin;
use cursor_hero_tools::ToolsPlugin;
use cursor_hero_ui::fps_text_plugin::FpsTextPlugin;
use cursor_hero_ui::position_text_plugin::PositionTextPlugin;
use cursor_hero_icon::IconPlugin;

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app/*¶*/
            .add_plugins(ToolsPlugin)
            .add_plugins(ToolbeltPlugin)
            .add_plugins(CharacterPlugin)
            .add_plugins(ActiveInputStatePlugin)
            .add_plugins(AfterimagePlugin)
            .add_plugins(CameraPlugin)
            // .add_plugins(ClickDragMovementPlugin)
            .add_plugins(FpsTextPlugin)
            // .add_plugins(HoverShowerRelayPlugin)
            // .add_plugins(HoverShowerServicePlugin)
            .add_plugins(PositionTextPlugin)
            .add_plugins(PressurePlatePlugin)
            .add_plugins(ScreenPlugin)
            .add_plugins(ScreenUpdatePlugin)
            .add_plugins(CursorMirroringPlugin)
            .add_plugins(HoverUiAutomationPlugin)
            .add_plugins(PointerPlugin)
            .add_plugins(DampingPlugin)
            .add_plugins(LevelBoundsPlugin)
            .add_plugins(PrimaryWindowMemoryPlugin)
            .add_plugins(IconPlugin)
            /*¶*/;
    }
}
