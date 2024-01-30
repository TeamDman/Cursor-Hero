use super::types::*;
use bevy::prelude::*;
use cursor_hero_pointer::pointer_plugin::Pointer;
use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintData;
use leafwing_input_manager::action_state::ActionState;

#[allow(clippy::type_complexity)]
pub fn pointer_reach(
    toolbelts: Query<(&ActionState<ToolbeltAction>, &Wheel, &Parent), With<Toolbelt>>,
    wearer_query: Query<&Children>,
    mut pointer_query: Query<&mut SprintData, With<Pointer>>,
) {
    for (toolbelt_actions, wheel, toolbelt_parent) in toolbelts.iter() {
        if toolbelt_actions.just_pressed(ToolbeltAction::Show) {
            if let Ok(wearer) = wearer_query.get(**toolbelt_parent) {
                for kid in wearer.iter() {
                    if let Ok(mut pointer_reach) = pointer_query.get_mut(*kid) {
                        pointer_reach.value = wheel.radius;
                        info!("Updated value to match wheel radius ({})", wheel.radius);
                    }
                }
            }
        } else if toolbelt_actions.just_released(ToolbeltAction::Show) {
            if let Ok(wearer) = wearer_query.get(**toolbelt_parent) {
                for kid in wearer.iter() {
                    if let Ok(mut pointer) = pointer_query.get_mut(*kid) {
                        pointer.value = pointer.default_value;
                        info!("Updated value to default reach ({})", pointer.default_value);
                    }
                }
            }
        }
    }
}
