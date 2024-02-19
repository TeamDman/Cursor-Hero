use crate::taskbar_spawn_plugin::TaskbarSpawnPlugin;
use bevy::prelude::*;

pub struct TaskbarPlugin;

impl Plugin for TaskbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TaskbarSpawnPlugin);
    }
}
