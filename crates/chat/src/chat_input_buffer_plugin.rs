use bevy::prelude::*;
use cursor_hero_character_types::character_types::Character;
use cursor_hero_chat_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;

pub struct ChatInputBufferPlugin;

impl Plugin for ChatInputBufferPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input);
    }
}

fn handle_input(
    mut tool_query: Query<(Entity, &mut ChatTool, &Parent), With<ActiveTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    character_query: Query<Entity, With<Character>>,
    mut chat_input_events: EventWriter<ChatInputEvent>,
    mut character_events: EventReader<ReceivedCharacter>,
) {
    for character_event in character_events.read() {
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
            tool.buffer.push(character_event.char);
            chat_input_events.send(ChatInputEvent::TextChanged {
                tool_id,
                toolbelt_id: tool_parent.get(),
                character_id,
            });
            debug!("Chat buffer: {}", tool.buffer);
        }
    }
}
