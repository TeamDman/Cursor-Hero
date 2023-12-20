use bevy::prelude::*;

use super::toolbar_plugin;
use super::toolbar_plugin::Tool;

pub struct PlaceholderToolPlugin;

impl Plugin for PlaceholderToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlaceholderTool>()
            .add_systems(Startup, setup.before(toolbar_plugin::setup));
    }
}

#[derive(Component, Reflect)]
pub struct PlaceholderTool;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("textures/tool_placeholder.png");
    for i in 0..7 {
        info!("");
        commands.spawn((
            PlaceholderTool,
            Name::new(format!("Placeholder Tool {}", i)),
            Tool(handle.clone()),
        ));
    }
}
