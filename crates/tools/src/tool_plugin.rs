use bevy::prelude::*;

use crate::click_tool::ClickToolPlugin;
use crate::cube_tool::CubeToolPlugin;
use crate::cursor_tool::CursorToolPlugin;
use crate::default_wheel_tool::DefaultWheelToolPlugin;
use crate::focus_tool::FocusToolPlugin;
use crate::hello_tool::HelloToolPlugin;
use crate::keyboard_tool::KeyboardToolPlugin;
use crate::keyboard_wheel_tool::KeyboardWheelToolPlugin;
use crate::level_bounds_visibility_tool::LevelBoundsVisibilityToolPlugin;
use crate::observation_tool::ObservationToolPlugin;
// use crate::placeholder_tool::PlaceholderToolPlugin;
#[cfg(debug_assertions)]
use crate::restart_tool::RestartToolPlugin;
use crate::sprint_tool::SprintToolPlugin;
use crate::talk_tool::TalkToolPlugin;
// use crate::window_drag_tool::WindowDragToolPlugin;
use crate::zoom_tool::ZoomToolPlugin;

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CubeToolPlugin);
        // app.add_plugins(PlaceholderToolPlugin);
        app.add_plugins(ClickToolPlugin);
        app.add_plugins(CursorToolPlugin);
        app.add_plugins(TalkToolPlugin);
        app.add_plugins(ZoomToolPlugin);
        app.add_plugins(FocusToolPlugin);
        app.add_plugins(SprintToolPlugin);
        app.add_plugins(DefaultWheelToolPlugin);
        // app.add_plugins(WindowDragToolPlugin);
        app.add_plugins(KeyboardToolPlugin);
        app.add_plugins(KeyboardWheelToolPlugin);
        app.add_plugins(HelloToolPlugin);
        app.add_plugins(ObservationToolPlugin);
        app.add_plugins(LevelBoundsVisibilityToolPlugin);
        #[cfg(debug_assertions)]
        app.add_plugins(RestartToolPlugin);
    }
}
