use crate::taskbar_wheel_tool::TaskbarWheelToolPlugin;
use bevy::prelude::*;
pub struct TaskbarToolPlugin;

impl Plugin for TaskbarToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TaskbarWheelToolPlugin);
    }
}
