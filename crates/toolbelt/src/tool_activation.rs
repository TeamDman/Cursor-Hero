use cursor_hero_toolbelt_types::toolbelt_types::*;

use bevy::prelude::*;
use cursor_hero_cursor_types::prelude::*;

use leafwing_input_manager::action_state::ActionState;

#[allow(clippy::type_complexity)]
pub fn tool_activation(
    mut commands: Commands,
    hovered_query: Query<(Entity, Option<&ActiveTool>, Option<&Name>), With<Hovered>>,
    toolbelt_query: Query<(&ActionState<ToolbeltAction>, &Children)>,
    mut events: EventWriter<ToolActivationEvent>,
) {
    for (toolbelt_actions, toolbelt_kids) in toolbelt_query.iter() {
        if toolbelt_actions.just_released(ToolbeltAction::Show) {
            for (hovered_id, hovered_active, hovered_name) in toolbelt_kids
                .iter()
                .filter_map(|h| hovered_query.get(*h).ok())
            {
                if hovered_active.is_some() {
                    commands.entity(hovered_id).remove::<ActiveTool>();
                    events.send(ToolActivationEvent::Deactivate(hovered_id));
                    info!("Deactivating tool: {:?} ({:?})", hovered_name, hovered_id);
                } else {
                    commands.entity(hovered_id).insert(ActiveTool);
                    events.send(ToolActivationEvent::Activate(hovered_id));
                    info!("Activating tool: {:?} ({:?})", hovered_name, hovered_id);
                }
            }
        }
    }
}
