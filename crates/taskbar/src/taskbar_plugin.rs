use crate::game_screen_taskbar_plugin::GameScreenTaskbarPlugin;
use bevy::prelude::*;

pub struct TaskbarPlugin;

impl Plugin for TaskbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameScreenTaskbarPlugin);
    }
}
