use crate::prelude::*;
use bevy::{prelude::*, sprite::Material2dPlugin};

pub struct TaskbarTypesPlugin;

impl Plugin for TaskbarTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Taskbar>();
        app.add_event::<TaskbarEvent>();
        app.add_plugins(Material2dPlugin::<TaskbarMaterial>::default());
    }
}
