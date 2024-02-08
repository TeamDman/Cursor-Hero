use bevy::prelude::*;
use cursor_hero_chat_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;

pub struct ChatFocusExclusivityPlugin;

impl Plugin for ChatFocusExclusivityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_focus_changed);
    }
}

fn handle_focus_changed(
    mut commands: Commands,
    mut events: EventReader<ChatInputEvent>,
    mut tool_query: Query<&mut ChatTool, With<ChatTool>>,
    toolbelt_query: Query<&Children, With<Toolbelt>>,
    active_tool_query: Query<(), (With<ActiveTool>, Without<ChatTool>)>,
) {
    for event in events.read() {
        let (ChatInputEvent::Focus {
            tool_id,
            toolbelt_id,
            ..
        }
        | ChatInputEvent::Unfocus {
            tool_id,
            toolbelt_id,
            ..
        }) = event
        else {
            continue;
        };
        let focusing = matches!(event, ChatInputEvent::Focus { .. });

        let Ok(tool) = tool_query.get_mut(*tool_id) else {
            warn!("Tool {:?} not found for event {:?}", tool_id, event);
            continue;
        };
        let mut tool = tool;

        let Ok(toolbelt) = toolbelt_query.get(*toolbelt_id) else {
            warn!("Toolbelt {:?} not found for event {:?}", toolbelt_id, event);
            continue;
        };
        let toolbelt_children = toolbelt;
        if focusing {
            tool.focused = true;
            for tool_id in toolbelt_children.iter() {
                if active_tool_query.contains(*tool_id) {
                    tool.tools_disabled_during_focus.insert(*tool_id);
                    commands.entity(*tool_id).remove::<ActiveTool>();
                    debug!("Disabled tool {:?} while focused", tool_id);
                }
            }
            debug!("Set tool {:?} as focused", tool_id);
        } else {
            tool.focused = false;
            for tool_id in tool.tools_disabled_during_focus.iter() {
                match commands.get_entity(*tool_id) {
                    Some(mut entity) => {
                        entity.insert(ActiveTool);
                    }
                    None => {
                        warn!("Error re-enabling tool {:?}, does not exist", tool_id);
                    }
                }
                debug!("Re-enabled tool {:?} after unfocusing", tool_id);
            }
            tool.tools_disabled_during_focus.clear();
            debug!("Set tool {:?} as unfocused", tool_id);
        }
    }
}
