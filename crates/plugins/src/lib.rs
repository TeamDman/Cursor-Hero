use bevy::prelude::*;

use cursor_hero_camera::camera_plugin::CameraPlugin;
use cursor_hero_character::character_plugin::CharacterPlugin;
use cursor_hero_character_movement::CharacterMovementPlugin;
use cursor_hero_cursor_mirror::cursor_mirroring_plugin::CursorMirroringPlugin;
use cursor_hero_hover::afterimage_plugin::AfterimagePlugin;
use cursor_hero_hover::hover_ui_automation_plugin::HoverUiAutomationPlugin;
use cursor_hero_input::active_input_state_plugin::ActiveInputStatePlugin;
use cursor_hero_level_bounds::level_bounds_plugin::LevelBoundsPlugin;
// use cursor_hero_click_drag_character_movement::ClickDragMovementPlugin;
use cursor_hero_icon::IconPlugin;
use cursor_hero_pause_tool::pause_tool_plugin::PauseToolPlugin;
use cursor_hero_physics::damping_plugin::DampingPlugin;
use cursor_hero_pointer::pointer_plugin::PointerPlugin;
use cursor_hero_pressure_plate::pressure_plate_plugin::PressurePlatePlugin;
#[cfg(debug_assertions)]
use cursor_hero_restart_memory::primary_window_memory_plugin::PrimaryWindowMemoryPlugin;
use cursor_hero_screen::screen_plugin::ScreenPlugin;
use cursor_hero_screen::screen_update_plugin::ScreenUpdatePlugin;
use cursor_hero_toolbelt::toolbelt_plugin::ToolbeltPlugin;
use cursor_hero_tools::ToolPlugin;
use cursor_hero_ui::about_text_plugin::AboutTextPlugin;
use cursor_hero_ui::fps_text_plugin::FpsTextPlugin;
use cursor_hero_ui::position_text_plugin::PositionTextPlugin;
use cursor_hero_wallpaper::wallpaper_plugin::WallpaperPlugin;
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PauseToolPlugin);
        app.add_plugins(WallpaperPlugin);
        app.add_plugins(ToolPlugin);
        app.add_plugins(ToolbeltPlugin);
        app.add_plugins(CharacterPlugin);
        app.add_plugins(CharacterMovementPlugin);
        app.add_plugins(ActiveInputStatePlugin);
        app.add_plugins(AfterimagePlugin);
        app.add_plugins(CameraPlugin);
        app.add_plugins(FpsTextPlugin);
        app.add_plugins(PositionTextPlugin);
        app.add_plugins(AboutTextPlugin);
        app.add_plugins(PressurePlatePlugin);
        app.add_plugins(ScreenPlugin);
        app.add_plugins(ScreenUpdatePlugin);
        app.add_plugins(CursorMirroringPlugin);
        app.add_plugins(HoverUiAutomationPlugin);
        app.add_plugins(PointerPlugin);
        app.add_plugins(DampingPlugin);
        app.add_plugins(LevelBoundsPlugin);
        app.add_plugins(IconPlugin);
        //app.add_plugins(ClickDragMovementPlugin);
        //app.add_plugins(HoverShowerRelayPlugin);
        //app.add_plugins(HoverShowerServicePlugin);

        #[cfg(debug_assertions)]
        app.add_plugins(PrimaryWindowMemoryPlugin);
    }
}
