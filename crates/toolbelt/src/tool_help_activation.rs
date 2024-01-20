use super::types::*;
use bevy::prelude::*;
use cursor_hero_bevy::NameOrEntityDisplay;
use leafwing_input_manager::action_state::ActionState;

pub fn tool_help_activation(
    toolbelt_query: Query<(&ActionState<ToolbeltAction>, &Children)>,
    tool_query: Query<(Option<&Name>, &Children), With<Tool>>,
    hovered_query: Query<Entity, (With<Hovered>, With<ToolHelpTrigger>)>,
    mut events: EventWriter<ToolActivationEvent>,
) {
    for (toolbelt_actions, toolbelt_children) in toolbelt_query.iter() {
        if toolbelt_actions.just_released(ToolbeltAction::Show) {
            // check all the toolbelt children
            for tool_id in toolbelt_children {
                // if the child is a tool
                if let Ok((tool_name, tool_children)) = tool_query.get(*tool_id) {
                    // and the tool has children
                    for tool_child_id in tool_children.iter() {
                        // and the hovered child is a tool help trigger
                        if let Ok(hovered_id) = hovered_query.get(*tool_child_id) {
                            // then activate the help for the hovered tool
                            events.send(ToolActivationEvent::ActivateHelp(hovered_id));
                            info!(
                                "Activating help for tool: {:?}",
                                tool_name.name_or_entity(hovered_id)
                            );
                        }
                    }
                }
            }
        }
    }
}
