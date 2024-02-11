use bevy::prelude::*;
use cursor_hero_inference_types::prelude::*;
use cursor_hero_observation_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::prelude::*;

pub struct ObservationToolPlugin;

impl Plugin for ObservationToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, tool_tick);
        app.add_systems(Update, reply_tick);
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
                .guess_image(file!(), &asset_server, "png")
                .with_description("Logs information about the environment to the console.")
                .with_starting_state(StartingState::Inactive)
                .spawn(&mut commands);
        }
    }
}

// TODO: rework this into the "inference wand" which will read the observation buffer in the target and send it over

#[allow(clippy::type_complexity)]
fn tool_tick(
    mut commands: Commands,
    tool_query: Query<(Entity, &Parent), (Added<ActiveTool>, With<ObservationTool>)>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut character_query: Query<&mut ObservationBuffer>,
    mut events: EventWriter<InferenceEvent>,
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
        let Ok(character) = character_query.get_mut(character_id) else {
            warn!("Failed to get character");
            continue;
        };
        let mut character_observation_buffer = character;

        let mut chat_history = String::new();
        for entry in character_observation_buffer.observations.iter() {
            let timestamp = entry.datetime.format("%Y-%m-%d %H:%M:%S").to_string();
            chat_history.push_str(&format!("{}: {}\n", timestamp, entry.observation));
        }
        character_observation_buffer.observations.clear();

        events.send(InferenceEvent::Request {
            session_id: character_id,
            prompt: Prompt::Chat { chat_history },
        });
        debug!("ObservationToolPlugin: Sent observation event");
    }
}

fn reply_tick(mut inference_events: EventReader<InferenceEvent>) {
    for event in inference_events.read() {
        let InferenceEvent::Response { response, .. } = event else {
            continue;
        };
        info!("ObservationToolPlugin: Received response: {}", response);
    }
}
