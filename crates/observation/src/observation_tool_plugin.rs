use bevy::prelude::*;
use cursor_hero_observation_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::prelude::*;

pub struct ObservationToolPlugin;

impl Plugin for ObservationToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, tick);
    }
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Inspector { toolbelt_id }
        | PopulateToolbeltEvent::Agent { toolbelt_id } = event
        {
            ToolSpawnConfig::<ObservationTool, NoInputs>::new(ObservationTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Logs information about the environment to the console.")
                .with_starting_state(StartingState::Inactive)
                .spawn(&mut commands);
        }
    }
}

fn tick(
    mut commands: Commands,
    tool_query: Query<(Entity, &Parent), (Added<ActiveTool>, With<ObservationTool>)>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut events: EventWriter<ObservationEvent>,
) {
    for tool in tool_query.iter() {
        let (tool_id, tool_parent) = tool;
        commands.entity(tool_id).remove::<ActiveTool>();

        let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
            warn!("Failed to get toolbelt");
            continue;
        };
        let toolbelt_parent = toolbelt;
        let character_id = toolbelt_parent.get();

        let observation = "Hello, world!".to_string();
        events.send(ObservationEvent::ObservationToolResponse {
            observation,
            character_id,
        });
        debug!("ObservationToolPlugin: Sent observation event");
    }
}
