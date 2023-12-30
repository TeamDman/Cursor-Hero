use bevy::prelude::*;

use super::{
    click_tool_plugin::ClickToolPlugin, cube_tool_plugin::CubeToolPlugin,
    placeholder_tool_plugin::PlaceholderToolPlugin,
    pointer_screen_position_tool::PointerScreenPositionToolPlugin,
    pointer_window_position_tool::PointerWindowPositionToolPlugin,
};

pub struct ToolsPlugin;
impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CubeToolPlugin,
            PlaceholderToolPlugin,
            ClickToolPlugin,
            PointerWindowPositionToolPlugin,
            PointerScreenPositionToolPlugin,
        ));
    }
}
