use bevy::prelude::*;

use super::{
    click_tool_plugin::ClickToolPlugin, cube_tool_plugin::CubeToolPlugin,
    focus_tool::FocusToolPlugin, inspect_tool::InspectToolPlugin, 
    placeholder_tool_plugin::PlaceholderToolPlugin,
    pointer_screen_position_tool::PointerScreenPositionToolPlugin,
    pointer_window_position_tool::PointerWindowPositionToolPlugin, restart_tool::RestartToolPlugin,
    talk_tool::TalkToolPlugin, zoom_tool::ZoomToolPlugin,
};

pub struct ToolsPlugin;
impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app/*¶*/
            .add_plugins(CubeToolPlugin)
            .add_plugins(PlaceholderToolPlugin)
            .add_plugins(ClickToolPlugin)
            .add_plugins(PointerWindowPositionToolPlugin)
            .add_plugins(PointerScreenPositionToolPlugin)
            .add_plugins(TalkToolPlugin)
            .add_plugins(ZoomToolPlugin)
            .add_plugins(FocusToolPlugin)
            .add_plugins(RestartToolPlugin)
            .add_plugins(InspectToolPlugin)
            /*¶*/
        ;
    }
}
