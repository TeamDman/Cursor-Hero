use bevy::prelude::*;

use crate::cube_tool::CubeToolPlugin;
use crate::default_wheel_tool::DefaultWheelToolPlugin;
use crate::focus_tool::FocusToolPlugin;
use crate::keyboard_tool::KeyboardToolPlugin;
use crate::keyboard_wheel_tool::KeyboardWheelToolPlugin;
use crate::level_bounds_visibility_tool::LevelBoundsVisibilityToolPlugin;
// use crate::placeholder_tool::PlaceholderToolPlugin;
#[cfg(debug_assertions)]
use crate::restart_tool::RestartToolPlugin;
use crate::scroll_tool::ScrollToolPlugin;
use crate::talk_tool::TalkToolPlugin;
// use crate::window_drag_tool::WindowDragToolPlugin;

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CubeToolPlugin);
        // app.add_plugins(PlaceholderToolPlugin);
        app.add_plugins(ScrollToolPlugin);
        app.add_plugins(TalkToolPlugin);
        app.add_plugins(FocusToolPlugin);
        app.add_plugins(DefaultWheelToolPlugin);
        // app.add_plugins(WindowDragToolPlugin);
        app.add_plugins(KeyboardToolPlugin);
        app.add_plugins(KeyboardWheelToolPlugin);
        app.add_plugins(LevelBoundsVisibilityToolPlugin);
        #[cfg(debug_assertions)]
        app.add_plugins(RestartToolPlugin);
    }
}
