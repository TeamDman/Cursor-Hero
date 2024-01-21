use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_toolbelt::types::*;

pub struct DefaultWheelToolPlugin;

impl Plugin for DefaultWheelToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DefaultWheelTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(Update, tick);
    }
}

#[derive(Component, Reflect)]
struct DefaultWheelTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        if let ToolbeltEvent::PopulateInspectorToolbelt {
            toolbelt_id,
            character_id,
        } = e
        {
            spawn_tool(
                Tool::create(
                    file!(),
                    "Swaps to default tools".to_string(),
                    &asset_server,
                ),
                e,
                &mut commands,
                *toolbelt_id,
                *character_id,
                &asset_server,
                DefaultWheelTool,
                StartingState::Inactive,
            );
        }
    }
}

fn tick(
    mut commands: Commands,
    tool_query: Query<&Parent, (Added<ActiveTool>, With<DefaultWheelTool>)>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut toolbelt_events: EventWriter<ToolbeltEvent>,
) {
    for toolbelt_id in tool_query.iter() {
        let toolbelt_id = toolbelt_id.get();
        if let Ok(character_id) = toolbelt_query.get(toolbelt_id) {
            let character_id = character_id.get();
            commands.entity(toolbelt_id).despawn_descendants();
            toolbelt_events.send(ToolbeltEvent::PopulateDefaultToolbelt {
                toolbelt_id: toolbelt_id,
                character_id: character_id,
            });
        }
    }
}
