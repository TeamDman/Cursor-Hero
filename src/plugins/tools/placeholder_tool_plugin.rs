use bevy::prelude::*;

use super::toolbar_plugin::Tool;
use super::toolbar_plugin::ToolbarSystemSet;

pub struct PlaceholderToolPlugin;

impl Plugin for PlaceholderToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlaceholderTool>()
            .add_systems(Startup, setup.before(ToolbarSystemSet::Spawn));
    }
}

#[derive(Component, Reflect)]
pub struct PlaceholderTool;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("textures/tool_placeholder.png");
    for i in 0..7 {
        commands.spawn((
            PlaceholderTool,
            Name::new(format!("Placeholder Tool {}", i)),
            Tool(handle.clone()),
        ));
    }
}
