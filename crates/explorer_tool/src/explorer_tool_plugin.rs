use crate::{explorer_tool_populate_plugin::ExplorerToolPopulatePlugin, explorer_tool_tick_plugin::ExplorerToolTickPlugin};
use bevy::prelude::*;

pub struct ExplorerToolPlugin;

impl Plugin for ExplorerToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExplorerToolPopulatePlugin);
        app.add_plugins(ExplorerToolTickPlugin);
    }
}
