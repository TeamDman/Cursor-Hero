use crate::zoom_tool_populate_plugin::ZoomToolPopulatePlugin;
use crate::zoom_tool_tick_plugin::ZoomToolTickPlugin;
use bevy::prelude::*;

pub struct ZoomToolPlugin;

impl Plugin for ZoomToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ZoomToolPopulatePlugin);
        app.add_plugins(ZoomToolTickPlugin);
    }
}
