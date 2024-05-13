use bevy::prelude::*;
use crate::{{crate_name}}_tool_populate_plugin::{{crate_name_pascal}}ToolPopulatePlugin;
use crate::{{crate_name}}_tool_populate_plugin::{{crate_name_pascal}}ToolTickPlugin;

pub struct {{crate_name_pascal}}ToolPlugin;

impl Plugin for {{crate_name_pascal}}ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins({{crate_name_pascal}}ToolPopulatePlugin);
        app.add_plugins({{crate_name_pascal}}ToolTickPlugin);
    }
}