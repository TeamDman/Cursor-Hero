use bevy::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_chat_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::prelude::*;
use cursor_hero_voice_to_text_types::voice_to_text_types::VoiceToTextTranscriptionEvent;
use leafwing_input_manager::prelude::*;

pub struct ChatToolPlugin;

impl Plugin for ChatToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ChatToolAction>::default());
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, handle_input);
        app.add_systems(Update, handle_voice_events);
    }
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        let (ToolbeltLoadout::Chat | ToolbeltLoadout::Default) = event.loadout else {
            continue;
        };
        {
            ToolSpawnConfig::<ChatTool, ChatToolAction>::new(ChatTool::default(), event.id, event)
                .with_src_path(file!().into())
                .guess_name(file!())
                .guess_image(file!(), &asset_server, "webp")
                .with_description("Send chat messages into the world")
                .spawn(&mut commands);
        }
    }
}

fn handle_input(
    mut tool_query: Query<
        (Entity, &ActionState<ChatToolAction>, &Parent, &mut ChatTool),
        With<ActiveTool>,
    >,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut character_query: Query<Entity, With<Character>>,
    mut chat_events: EventWriter<ChatEvent>,
    mut chat_input_events: EventWriter<ChatInputEvent>,
) {
    for tool in tool_query.iter_mut() {
        let (tool_id, tool_actions, tool_parent, mut tool) = tool;

        let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
            warn!("Tool not inside a toolbelt?");
            continue;
        };
        let toolbelt_parent = toolbelt;
        let Ok(character) = character_query.get_mut(toolbelt_parent.get()) else {
            warn!("Toolbelt parent not a character?");
            continue;
        };
        let character_id = character;

        if tool_actions.just_pressed(ChatToolAction::Focus) && !tool.focused {
            let event = ChatInputEvent::Focus {
                tool_id,
                toolbelt_id: tool_parent.get(),
                character_id,
            };
            info!("Sending focus event {:?}", event);
            chat_input_events.send(event);
        } else if tool_actions.just_pressed(ChatToolAction::Unfocus) && tool.focused {
            let event = ChatInputEvent::Unfocus {
                tool_id,
                toolbelt_id: tool_parent.get(),
                character_id,
            };
            info!("Sending unfocus event {:?}", event);
            chat_input_events.send(event);
        } else if tool_actions.just_pressed(ChatToolAction::Submit) && tool.focused {
            let message = tool.buffer.clone();
            if !message.is_empty() {
                tool.buffer.clear();

                let event = ChatEvent::Chat {
                    character_id,
                    message,
                };
                info!("Sending chat event {:?}", event);
                chat_events.send(event);
            }
            let event = ChatInputEvent::Unfocus {
                tool_id,
                toolbelt_id: tool_parent.get(),
                character_id,
            };
            info!("Sending unfocus event {:?}", event);
            chat_input_events.send(event);
        }
    }
}

fn handle_voice_events(
    mut voice_events: EventReader<VoiceToTextTranscriptionEvent>,
    mut chat_events: EventWriter<ChatEvent>,
    character_query: Query<Entity, With<MainCharacter>>,
) {
    let character_id = match character_query.get_single() {
        Ok(character_id) => character_id,
        Err(e) => {
            warn!("Failed to get main character: {:?}", e);
            return;
        }
    };

    for event in voice_events.read() {
        let VoiceToTextTranscriptionEvent::Received { transcription } = event;
        if transcription.is_empty() {
            continue;
        }
        let event = ChatEvent::Chat {
            character_id,
            message: transcription.clone(),
        };
        info!("Sending chat event {:?}", event);
        chat_events.send(event);
    }
}
