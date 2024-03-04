use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_inference_types::inference_types::SpeechInferenceEvent;
use cursor_hero_inference_types::inference_types::TextInferenceEvent;
use cursor_hero_inference_types::inference_types::TextInferenceOptions;
use cursor_hero_inference_types::prompt_types::SpeechPrompt;
use cursor_hero_inference_types::prompt_types::TextPrompt;
use cursor_hero_toolbelt_types::prelude::*;

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
        if event.loadout == ToolbeltLoadout::Inspector {
            ToolSpawnConfig::<HelloTool, NoInputs>::new(HelloTool, event.id, event)
                .with_src_path(file!().into())
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
    mut inference_events: EventWriter<TextInferenceEvent>,
) {
    for tool_id in tool_query.iter() {
        commands.entity(tool_id).remove::<ActiveTool>();
        info!("Hello, world!");
        inference_events.send(TextInferenceEvent::Request {
            session_id: tool_id,
            prompt: TextPrompt::Raw {
                content: "Here is a random word:".to_string(),
                options: Some(TextInferenceOptions {
                    num_predict: Some(7),
                    ..default()
                }),
            },
        });
    }
}

fn inference_response(
    mut llm_events: EventReader<TextInferenceEvent>,
    mut tts_events: EventWriter<SpeechInferenceEvent>,
    tool_query: Query<Entity, With<HelloTool>>,
) {
    for event in llm_events.read() {
        if let TextInferenceEvent::Response {
            session_id,
            response,
            prompt: _,
        } = event
        {
            if tool_query.get(*session_id).is_ok() {
                tts_events.send(SpeechInferenceEvent::Request {
                    session_id: *session_id,
                    prompt: SpeechPrompt::Raw {
                        content: response.clone(),
                    },
                });
            }
        }
    }
}

fn tts_response(
    mut commands: Commands,
    mut tts_events: EventReader<SpeechInferenceEvent>,
    tool_query: Query<Entity, With<HelloTool>>,
    mut audio_assets: ResMut<Assets<AudioSource>>,
) {
    for event in tts_events.read() {
        if let SpeechInferenceEvent::Response {
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
