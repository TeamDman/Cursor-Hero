use std::time::Duration;

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use cursor_hero_character_types::character_types::Character;
use cursor_hero_chat_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use leafwing_input_manager::action_state::ActionState;

pub struct ChatInputBufferPlugin;

impl Plugin for ChatInputBufferPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_char);
        app.add_systems(Update, handle_input);
    }
}

fn handle_char(
    mut tool_query: Query<(Entity, &mut ChatTool, &Parent), With<ActiveTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    character_query: Query<Entity, With<Character>>,
    mut chat_input_events: EventWriter<ChatInputEvent>,
    mut character_events: EventReader<ReceivedCharacter>,
) {
    for event in character_events.read() {
        if event.char.is_control() {
            continue;
        }
        for tool in tool_query.iter_mut() {
            let (tool_id, mut tool, tool_parent) = tool;
            if !tool.focused {
                continue;
            }
            let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
                warn!("Tool not inside a toolbelt?");
                continue;
            };
            let toolbelt_parent = toolbelt;
            let Ok(character) = character_query.get(toolbelt_parent.get()) else {
                warn!("Toolbelt parent not a character?");
                continue;
            };
            let character_id = character;
            tool.buffer.push(event.char);
            chat_input_events.send(ChatInputEvent::TextChanged {
                tool_id,
                toolbelt_id: tool_parent.get(),
                character_id,
            });
            debug!(
                "Appended char '{}' ({}) to chat buffer. New: {}",
                event.char, event.char as u32, tool.buffer
            );
        }
    }
}

fn handle_input(
    mut tool_query: Query<
        (Entity, &mut ChatTool, &ActionState<ChatToolAction>, &Parent),
        With<ActiveTool>,
    >,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    character_query: Query<Entity, With<Character>>,
    mut chat_input_events: EventWriter<ChatInputEvent>,
    time: Res<Time>,
) {
    for tool in tool_query.iter_mut() {
        let (tool_id, mut tool, tool_actions, tool_parent) = tool;
        if !tool.focused {
            continue;
        }
        let input_active = tool_actions.pressed(ChatToolAction::Backspace);
        match (input_active, &mut tool.state) {
            // no input is active, return to default state
            (false, state) => {
                if *state != ChatToolState::Idle {
                    tool.state = ChatToolState::Idle;
                }
                continue;
            }
            // first input, start initial delay
            (true, ChatToolState::Idle) => {
                tool.state =
                    ChatToolState::InitialRepeatDelay(Timer::from_seconds(0.5, TimerMode::Once));
            }
            // check initial delay finished, start repeat delay
            (true, ChatToolState::InitialRepeatDelay(ref mut timer)) => {
                if timer.tick(time.delta()).just_finished() {
                    tool.state =
                        ChatToolState::RepeatDelay(Timer::from_seconds(0.03, TimerMode::Repeating));
                } else {
                    continue;
                }
            }
            // been held, continue repeat delay
            (true, ChatToolState::RepeatDelay(ref mut timer)) => {
                if !timer.tick(time.delta()).just_finished() {
                    continue;
                }
            }
        }

        let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
            warn!("Tool not inside a toolbelt?");
            continue;
        };
        let toolbelt_parent = toolbelt;
        let Ok(character) = character_query.get(toolbelt_parent.get()) else {
            warn!("Toolbelt parent not a character?");
            continue;
        };
        let character_id = character;
        let original_buffer = tool.buffer.clone();
        if tool_actions.pressed(ChatToolAction::Backspace) {
            if tool_actions.pressed(ChatToolAction::WordModifier) {
                // delete word
                while let Some(c) = tool.buffer.pop() {
                    if c.is_whitespace() {
                        break;
                    }
                }
            } else {
                tool.buffer.pop();
            }
        }
        if original_buffer == tool.buffer {
            continue;
        }
        chat_input_events.send(ChatInputEvent::TextChanged {
            tool_id,
            toolbelt_id: tool_parent.get(),
            character_id,
        });
        debug!("Updated chat buffer. New: {}", tool.buffer);
    }
}
