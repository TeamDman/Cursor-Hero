use crate::prelude::*;
use bevy::prelude::*;

pub struct TaskbarTypesPlugin;

impl Plugin for TaskbarTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Taskbar>();
        app.add_event::<TaskbarEvent>();
    }
}
