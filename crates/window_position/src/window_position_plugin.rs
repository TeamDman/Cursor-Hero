use bevy::prelude::*;

use crate::window_position_loadout_switcher_tool_plugin::WindowPositionLoadoutSwitcherToolPlugin;
use crate::window_position_tool_plugin::WindowPositionToolPlugin;

pub struct WindowPositionPlugin;

impl Plugin for WindowPositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowPositionLoadoutSwitcherToolPlugin);
        app.add_plugins(WindowPositionToolPlugin);
    }
}
