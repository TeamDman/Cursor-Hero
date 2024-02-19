use bevy::prelude::*;
use crate::taskbar_wheel_tool::TaskbarWheelToolPlugin;
pub struct TaskbarToolPlugin;

impl Plugin for TaskbarToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TaskbarWheelToolPlugin);
    }
}