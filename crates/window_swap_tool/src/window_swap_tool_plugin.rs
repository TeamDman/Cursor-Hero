use crate::window_swap_tool_populate_plugin::WindowSwapToolPopulatePlugin;
use crate::window_swap_tool_tick_plugin::WindowSwapToolTickPlugin;
use bevy::prelude::*;

pub struct WindowSwapToolPlugin;

impl Plugin for WindowSwapToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowSwapToolPopulatePlugin);
        app.add_plugins(WindowSwapToolTickPlugin);
    }
}
