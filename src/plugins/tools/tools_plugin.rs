use bevy::prelude::*;

use super::{cube_tool_plugin::CubeToolPlugin, placeholder_tool_plugin::PlaceholderToolPlugin};

pub struct ToolsPlugin;
impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CubeToolPlugin,
            PlaceholderToolPlugin,
        ));
    }
}
