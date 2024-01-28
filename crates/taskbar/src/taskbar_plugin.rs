use crate::screen_taskbar_plugin::ScreenTaskbarPlugin;
use crate::taskbar_wheel_tool::TaskbarWheelToolPlugin;
use bevy::prelude::*;

pub struct TaskbarPlugin;

impl Plugin for TaskbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TaskbarWheelToolPlugin);
        app.add_plugins(ScreenTaskbarPlugin);
    }
}
