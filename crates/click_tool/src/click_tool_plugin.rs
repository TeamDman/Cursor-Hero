use crate::click_tool_populate_plugin::ClickToolPopulatePlugin;
use crate::click_tool_tick_plugin::ClickToolTickPlugin;
use bevy::prelude::*;

pub struct ClickToolPlugin;

impl Plugin for ClickToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ClickToolPopulatePlugin);
        app.add_plugins(ClickToolTickPlugin);
    }
}
