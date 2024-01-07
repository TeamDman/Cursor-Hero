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
        app/*¶*/
            .add_plugins(CubeToolPlugin)
            // .add_plugins(PlaceholderToolPlugin)
            .add_plugins(ClickToolPlugin)
            .add_plugins(PointerWindowPositionToolPlugin)
            .add_plugins(PointerScreenPositionToolPlugin)
            .add_plugins(TalkToolPlugin)
            .add_plugins(ZoomToolPlugin)
            .add_plugins(FocusToolPlugin)
            .add_plugins(RestartToolPlugin)
            .add_plugins(InspectToolPlugin)
            .add_plugins(SprintToolPlugin)
            /*¶*/
        ;
    }
}
