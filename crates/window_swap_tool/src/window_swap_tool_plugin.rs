use bevy::prelude::*;
use crate::window_swap_tool_populate_plugin::WindowSwapToolPopulatePlugin;

pub struct WindowSwapToolPlugin;

impl Plugin for WindowSwapToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowSwapToolPopulatePlugin);
    }
}