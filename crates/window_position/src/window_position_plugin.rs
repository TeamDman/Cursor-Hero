use bevy::prelude::*;

use crate::window_position_command_plugin::WindowPositionCommandPlugin;
use crate::window_position_loadout_switcher_tool_plugin::WindowPositionLoadoutSwitcherToolPlugin;
use crate::window_position_tool_plugin::WindowPositionToolPlugin;

pub struct WindowPositionPlugin;

impl Plugin for WindowPositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowPositionLoadoutSwitcherToolPlugin);
        app.add_plugins(WindowPositionToolPlugin);
        app.add_plugins(WindowPositionCommandPlugin);
    }
}
