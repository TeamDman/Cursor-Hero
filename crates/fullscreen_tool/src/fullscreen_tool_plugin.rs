use crate::fullscreen_tool_populate_plugin::FullscreenToolPopulatePlugin;
use crate::fullscreen_tool_tick_plugin::FullscreenToolTickPlugin;
use bevy::prelude::*;

pub struct FullscreenToolPlugin;

impl Plugin for FullscreenToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FullscreenToolPopulatePlugin);
        app.add_plugins(FullscreenToolTickPlugin);
    }
}
