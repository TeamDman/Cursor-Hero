pub mod cube_tool_plugin;
pub mod toolbar_plugin;

use self::cube_tool_plugin::CubeToolPlugin;
use self::toolbar_plugin::ToolbarPlugin;

use bevy::prelude::*;


pub struct ToolsPlugin;
impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CubeToolPlugin)
        .add_plugins(ToolbarPlugin);
    }
}