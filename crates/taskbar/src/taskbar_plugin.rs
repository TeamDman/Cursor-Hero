use crate::game_screen_taskbar_plugin::GameScreenTaskbarPlugin;
use crate::start_menu_button_plugin::StartMenuButtonPlugin;
use crate::start_menu_plugin::StartMenuPlugin;
use crate::taskbar_wheel_tool::TaskbarWheelToolPlugin;
use bevy::prelude::*;

pub struct TaskbarPlugin;

impl Plugin for TaskbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TaskbarWheelToolPlugin);
        app.add_plugins(GameScreenTaskbarPlugin);
        app.add_plugins(StartMenuButtonPlugin);
        app.add_plugins(StartMenuPlugin);
    }
}
