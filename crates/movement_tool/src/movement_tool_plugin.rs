use bevy::prelude::*;

use crate::movement_speed_plugin::MovementSpeedPlugin;
use crate::movement_sprint_plugin::MovementSprintPlugin;
use crate::movement_target_plugin::MovementTargetPlugin;
use crate::movement_tool_populate_plugin::MovementToolPopulatePlugin;
use crate::movement_tool_tick_plugin::MovementToolTickPlugin;

pub struct MovementToolPlugin;

impl Plugin for MovementToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MovementSpeedPlugin);
        app.add_plugins(MovementSprintPlugin);
        app.add_plugins(MovementToolTickPlugin);
        app.add_plugins(MovementTargetPlugin);
        app.add_plugins(MovementToolPopulatePlugin);
    }
}
