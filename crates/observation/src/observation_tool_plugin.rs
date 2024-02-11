use bevy::prelude::*;
use bevy::utils::Instant;
use cursor_hero_character_types::character_types::AgentCharacter;
use cursor_hero_chat_types::chat_types::ChatEvent;
use cursor_hero_inference_types::prelude::*;
use cursor_hero_observation_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::prelude::*;

pub struct ObservationToolPlugin;

impl Plugin for ObservationToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, tool_tick);
        app.add_systems(Update, handle_text_inference_response);
        app.add_systems(Update, handle_tts_inference_response);
    }
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Agent { toolbelt_id } = event {
            ToolSpawnConfig::<ObservationTool, NoInputs>::new(
                ObservationTool::default(),
                *toolbelt_id,
                event,
            )
            .guess_name(file!())
            .guess_image(file!(), &asset_server, "png")
            .with_description("Logs information about the environment to the console.")
            .spawn(&mut commands);
        }
    }
}

#[allow(clippy::type_complexity)]
fn tool_tick(
    mut tool_query: Query<(&Parent, &mut ObservationTool), With<ActiveTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut character_query: Query<&mut ObservationBuffer>,
    mut events: EventWriter<TextInferenceEvent>,
) {
    for tool in tool_query.iter_mut() {
        let (tool_parent, mut tool) = tool;

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

        let character_observation_buffer = character;
        let mut whats_new = WhatsNew::Nothing;
        if let Some(last_inference) = tool.last_inference {
            for entry in character_observation_buffer
                .observations
                .iter()
                .filter(|entry| entry.instant > last_inference)
            {
                let is_self_chat = matches!(entry.origin, ObservationEvent::Chat { character_id: event_character_id, .. } if event_character_id == character_id);
                if is_self_chat {
                    whats_new = WhatsNew::SelfChat;
                } else {
                    whats_new = WhatsNew::Something;
                    break;
                }
            }
        } else if !character_observation_buffer.observations.is_empty() {
            whats_new = WhatsNew::Something;
        }

        // Update the field for debug viewing in the inspector
        tool._whats_new = Some(whats_new);

        // the agent will observe its own chats
        // so this check doesn't prevent all forms of loops
        if let WhatsNew::Nothing = whats_new {
            continue;
        }

        if let Some(last_inference) = tool.last_inference {
            if last_inference + whats_new.cooldown() > Instant::now() {
                continue;
            }
        }

        let mut chat_history = String::new();
        for entry in character_observation_buffer.observations.iter() {
            // let timestamp = entry.datetime.format("%Y-%m-%d %H:%M:%S").to_string();
            chat_history.push_str(entry.observation.as_str());
            chat_history.push_str("\n");
        }

        events.send(TextInferenceEvent::Request {
            session_id: character_id,
            prompt: TextPrompt::Chat {
                chat_history,
                options: Some(TextInferenceOptions {
                    stop: Some(vec![
                        "\n".to_string(),
                        "(Human)".to_string(),
                        "(Tume Eena)".to_string(),
                        "(Ithia Tig)".to_string(),
                    ]),
                    ..default()
                }),
            },
        });
        debug!("ObservationToolPlugin: Sent observation event");

        tool.last_inference = Some(Instant::now());
    }
}

fn handle_text_inference_response(
    mut inference_events: EventReader<TextInferenceEvent>,
    mut chat_events: EventWriter<ChatEvent>,
    mut tts_events: EventWriter<SpeechInferenceEvent>,
    agent_query: Query<(), With<AgentCharacter>>,
) {
    for event in inference_events.read() {
        let TextInferenceEvent::Response {
            response,
            session_id,
            ..
        } = event
        else {
            continue;
        };
        if !agent_query.get(*session_id).is_ok() {
            // Only inference responses for agent sessions are to be converted to chat messages and spoken
            continue;
        }

        if response.is_empty() {
            debug!("Received empty response, skipping");
            continue;
        }

        let event = ChatEvent::Chat {
            character_id: *session_id,
            message: response.clone(),
        };
        debug!("Sending event: {:?}", event);
        chat_events.send(event);

        let event = SpeechInferenceEvent::Request {
            session_id: *session_id,
            prompt: SpeechPrompt::Raw {
                content: response.clone(),
            },
        };
        debug!("Sending event: {:?}", event);
        tts_events.send(event);
    }
}

fn handle_tts_inference_response(
    mut commands: Commands,
    mut tts_events: EventReader<SpeechInferenceEvent>,
    agent_query: Query<(), With<AgentCharacter>>,
    mut audio_assets: ResMut<Assets<AudioSource>>,
) {
    for event in tts_events.read() {
        if let SpeechInferenceEvent::Response {
            session_id, wav, ..
        } = event
        {
            if agent_query.get(*session_id).is_ok() {
                info!(
                    "Received TTS response for session {:?}, playing",
                    session_id
                );
                let audio = audio_assets.add(AudioSource {
                    bytes: wav.clone().into(),
                });
                commands.entity(*session_id).insert({
                    AudioBundle {
                        source: audio,
                        settings: PlaybackSettings::REMOVE.with_spatial(true),
                    }
                });
            }
        }
    }
}
