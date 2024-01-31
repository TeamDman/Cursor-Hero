use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;

pub struct HelloToolPlugin;

impl Plugin for HelloToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HelloTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(Update, tick);
    }
}

#[derive(Component, Reflect, Default)]
struct HelloTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Inspector { toolbelt_id } = event {
            ToolSpawnConfig::<HelloTool, NoInputs>::new(HelloTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Prints hello.")
                .with_starting_state(StartingState::Inactive)
                .spawn(&mut commands);
        }
    }
}

fn tick(mut commands: Commands, tool_query: Query<Entity, (Added<ActiveTool>, With<HelloTool>)>) {
    for tool_id in tool_query.iter() {
        commands.entity(tool_id).remove::<ActiveTool>();
        info!("Hello, world!");
    }
}
