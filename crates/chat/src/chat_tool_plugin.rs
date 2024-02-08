use bevy::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_chat_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ChatToolPlugin;

impl Plugin for ChatToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ChatToolAction>::default());
        app.add_systems(Update, (toolbelt_events, handle_input));
    }
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Chat { toolbelt_id }
        | PopulateToolbeltEvent::Default { toolbelt_id } = event
        {
            ToolSpawnConfig::<ChatTool, ChatToolAction>::new(
                ChatTool::default(),
                *toolbelt_id,
                event,
            )
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
    toolbelt_query: Query<(&Parent, &Children), With<Toolbelt>>,
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
        let (toolbelt_parent, toolbelt_children) = toolbelt;
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
            tool.buffer.clear();

            let event = ChatEvent::Chat {
                character_id,
                message,
            };
            info!("Sending chat event {:?}", event);
            chat_events.send(event);

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
