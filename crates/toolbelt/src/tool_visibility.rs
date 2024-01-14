use super::types::*;
use bevy::prelude::*;
use bevy_xpbd_2d::constraints::FixedJoint;
use cursor_hero_input::update_gamepad_settings::PRESS_THRESHOLD;
use itertools::Itertools;
use leafwing_input_manager::action_state::ActionState;

pub fn tool_visibility(
    mut toolbelts: Query<
        (&ActionState<ToolbeltAction>, &mut Wheel, &Children),
        (Without<Tool>, With<Toolbelt>),
    >,
    mut tool_query: Query<(Entity, &mut Transform, &mut Visibility), With<Tool>>,
) {
    for (toolbelt_actions, mut wheel, toolbelt_kids) in toolbelts.iter_mut() {
        if toolbelt_actions.just_pressed(ToolbeltAction::Show) {
            info!("Show toolbelt");
            for (_, _, mut tool_visibility) in tool_query.iter_mut() {
                *tool_visibility = Visibility::Visible;
            }
        } else if toolbelt_actions.just_released(ToolbeltAction::Show) {
            info!("Hide toolbelt");
            for (_, _, mut tool_visibility) in tool_query.iter_mut() {
                *tool_visibility = Visibility::Hidden;
            }
        }
    }
}
