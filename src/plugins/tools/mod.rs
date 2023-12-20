pub mod cube_tool_plugin;
pub mod placeholder_tool_plugin;
pub mod toolbar_plugin;

use self::cube_tool_plugin::CubeToolPlugin;
use self::placeholder_tool_plugin::PlaceholderToolPlugin;
use self::toolbar_plugin::ToolbarPlugin;

use bevy::prelude::*;

pub struct ToolsPlugin;
impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CubeToolPlugin, PlaceholderToolPlugin))
            .add_plugins(ToolbarPlugin);
    }
}
