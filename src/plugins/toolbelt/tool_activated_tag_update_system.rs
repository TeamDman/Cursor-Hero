use super::types::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

pub fn tool_activation_update_system(
    mut commands: Commands,
    hovered: Query<(Entity, Option<&ToolActiveTag>), With<ToolHoveredTag>>,
    toolbelts: Query<(&ActionState<ToolbeltAction>, &Children)>,
    mut events: EventWriter<ToolActivationEvent>,
) {
    for (t_act, t_kids) in toolbelts.iter() {
        if t_act.just_released(ToolbeltAction::Show) {
            for (h_e, h_active) in t_kids.iter().filter_map(|h| hovered.get(*h).ok()) {
                if h_active.is_some() {
                    commands.entity(h_e).remove::<ToolActiveTag>();
                    events.send(ToolActivationEvent::Deactivate(h_e));
                    info!("Deactivating tool: {:?}", h_e);
                } else {
                    commands.entity(h_e).insert(ToolActiveTag);
                    events.send(ToolActivationEvent::Activate(h_e));
                    info!("Activating tool: {:?}", h_e);
                }
            }
        }
    }
}
