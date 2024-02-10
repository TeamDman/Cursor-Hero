use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_inference_types::{inference_types::InferenceEvent, prompt_types::Prompt};
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tts_types::tts_types::TtsEvent;

pub struct HelloToolPlugin;

impl Plugin for HelloToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HelloTool>();
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, activation);
        app.add_systems(Update, inference_response);
        app.add_systems(Update, tts_response);
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
                .guess_image(file!(), &asset_server, "png")
                .with_description("Prints hello.")
                .with_starting_state(StartingState::Inactive)
                .spawn(&mut commands);
        }
    }
}

fn activation(
    mut commands: Commands,
    tool_query: Query<Entity, (Added<ActiveTool>, With<HelloTool>)>,
    mut inference_events: EventWriter<InferenceEvent>,
) {
    for tool_id in tool_query.iter() {
        commands.entity(tool_id).remove::<ActiveTool>();
        info!("Hello, world!");
        inference_events.send(InferenceEvent::Request {
            session_id: tool_id,
            prompt: Prompt::Raw { 
                content: "Hello, ".to_string(),
            },
        });
    }
}

fn inference_response(
    mut inference_events: EventReader<InferenceEvent>,
    mut tts_events: EventWriter<TtsEvent>,
    tool_query: Query<Entity, With<HelloTool>>,
) {
    for event in inference_events.read() {
        if let InferenceEvent::Response {
            session_id,
            response,
            prompt: Prompt::Raw { content },
        } = event
        {
            if tool_query.get(*session_id).is_ok() {
                tts_events.send(TtsEvent::Request {
                    session_id: *session_id,
                    prompt: format!("{}{}", content, response),
                });
            }
        }
    }
}

fn tts_response(
    mut commands: Commands,
    mut tts_events: EventReader<TtsEvent>,
    tool_query: Query<Entity, With<HelloTool>>,
    mut audio_assets: ResMut<Assets<AudioSource>>,
) {
    for event in tts_events.read() {
        if let TtsEvent::Response {
            session_id, wav, ..
        } = event
        {
            if tool_query.get(*session_id).is_ok() {
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
