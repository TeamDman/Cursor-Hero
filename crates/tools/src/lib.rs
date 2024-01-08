pub mod click_tool_plugin;
pub mod cube_tool_plugin;
pub mod focus_tool;
pub mod inspect_tool;
pub mod placeholder_tool_plugin;
pub mod pointer_screen_position_tool;
pub mod pointer_window_position_tool;
pub mod restart_tool;
pub mod sprint_tool_plugin;
pub mod talk_tool;
pub mod zoom_tool;
use bevy::prelude::*;

use crate::click_tool_plugin::ClickToolPlugin;
use crate::cube_tool_plugin::CubeToolPlugin;
use crate::focus_tool::FocusToolPlugin;
use crate::inspect_tool::InspectToolPlugin;
use crate::placeholder_tool_plugin::PlaceholderToolPlugin;
use crate::pointer_screen_position_tool::PointerScreenPositionToolPlugin;
use crate::pointer_window_position_tool::PointerWindowPositionToolPlugin;
use crate::restart_tool::RestartToolPlugin;
use crate::sprint_tool_plugin::SprintToolPlugin;
use crate::talk_tool::TalkToolPlugin;
use crate::zoom_tool::ZoomToolPlugin;

pub struct ToolsPlugin;
impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CubeToolPlugin);
        // app.add_plugins(PlaceholderToolPlugin);
        app.add_plugins(ClickToolPlugin);
        app.add_plugins(PointerWindowPositionToolPlugin);
        app.add_plugins(PointerScreenPositionToolPlugin);
        app.add_plugins(TalkToolPlugin);
        app.add_plugins(ZoomToolPlugin);
        app.add_plugins(FocusToolPlugin);
        app.add_plugins(RestartToolPlugin);
        app.add_plugins(InspectToolPlugin);
        app.add_plugins(SprintToolPlugin);
    }
}
