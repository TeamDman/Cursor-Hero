use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_toolbelt::types::*;

pub struct InspectWheelToolPlugin;

impl Plugin for InspectWheelToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InspectWheelTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(Update, tick);
    }
}

#[derive(Component, Reflect)]
struct InspectWheelTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        if let ToolbeltEvent::PopulateDefaultToolbelt {
            toolbelt_id,
            character_id,
        } = e
        {
            spawn_tool(
                Tool::create(
                    file!(),
                    "Swaps to inspection tools".to_string(),
                    &asset_server,
                ),
                e,
                &mut commands,
                *toolbelt_id,
                *character_id,
                &asset_server,
                InspectWheelTool,
                StartingState::Inactive,
            );
        }
    }
}

fn tick(
    mut commands: Commands,
    tool_query: Query<&Parent, (Added<ActiveTool>, With<InspectWheelTool>)>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut toolbelt_events: EventWriter<ToolbeltEvent>,
) {
    for toolbelt_id in tool_query.iter() {
        let toolbelt_id = toolbelt_id.get();
        if let Ok(character_id) = toolbelt_query.get(toolbelt_id) {
            let character_id = character_id.get();
            commands.entity(toolbelt_id).despawn_descendants();
            toolbelt_events.send(ToolbeltEvent::PopulateInspectorToolbelt {
                toolbelt_id: toolbelt_id,
                character_id: character_id,
            });
        }
    }
}
