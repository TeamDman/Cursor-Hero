use bevy::prelude::*;
use crate::zoom_tool_populate_plugin::ZoomToolPopulatePlugin;

pub struct ZoomToolPlugin;

impl Plugin for ZoomToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ZoomToolPopulatePlugin);
    }
}