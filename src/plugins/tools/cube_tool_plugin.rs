use bevy::prelude::*;

use super::toolbar_plugin::Tool;
use super::toolbar_plugin::ToolbarSystemSet;

pub struct CubeToolPlugin;

impl Plugin for CubeToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CubeTool>()
            .add_systems(Startup, setup.before(ToolbarSystemSet::Spawn));
    }
}

#[derive(Component, Reflect)]
pub struct CubeTool;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        CubeTool,
        Name::new("Cube Tool"),
        Tool(asset_server.load("textures/tool_bulb.png")),
    ));
    info!("Cube Tool setup");
}
