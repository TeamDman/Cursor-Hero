use bevy::prelude::*;
use crate::explorer_tool_populate_plugin::ExplorerToolPopulatePlugin;

pub struct ExplorerToolPlugin;

impl Plugin for ExplorerToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExplorerToolPopulatePlugin);
    }
}