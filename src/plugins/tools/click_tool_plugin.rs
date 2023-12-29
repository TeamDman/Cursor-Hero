use bevy::prelude::*;

pub struct ClickToolPlugin;

impl Plugin for ClickToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Startup, add_tool);
    }
}

