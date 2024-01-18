use super::types::*;
use bevy::prelude::*;
use cursor_hero_bevy::NameOrEntityDisplay;
use leafwing_input_manager::action_state::ActionState;

pub fn tool_help_activation(
    mut commands: Commands,
    hovered_query: Query<(Entity, Option<&Name>), With<Hovered>>,
    toolbelt_query: Query<(&ActionState<ToolbeltAction>, &Children)>,
    mut events: EventWriter<ToolActivationEvent>,
) {
    for (toolbelt_actions, toolbelt_children) in toolbelt_query.iter() {
        if toolbelt_actions.just_released(ToolbeltAction::Show) {
            for (hovered_id, hovered_name) in toolbelt_children
                .iter()
                .filter_map(|h| hovered_query.get(*h).ok())
            {
                events.send(ToolActivationEvent::ActivateHelp(hovered_id));
                info!(
                    "Activating help for tool: {:?}",
                    hovered_name.name_or_entity(hovered_id)
                );
            }
        }
    }
}
