use crate::wheel_distribution::distribute;

use super::types::*;
use bevy::prelude::*;
use bevy_xpbd_2d::constraints::FixedJoint;
use cursor_hero_input::update_gamepad_settings::PRESS_THRESHOLD;
use leafwing_input_manager::action_state::ActionState;

pub fn wheel_opening(
    mut toolbelts: Query<
        (
            &ActionState<ToolbeltAction>,
            &mut Visibility,
            &mut Wheel,
            &Children,
        ),
        Without<Tool>,
    >,
    mut tool_query: Query<&mut Transform, With<Tool>>,
    mut tool_joint_query: Query<&mut FixedJoint, With<ToolJoint>>,
) {
    for (toolbelt_actions, mut toolbelt_visibility, mut wheel, toolbelt_kids) in
        toolbelts.iter_mut()
    {
        if toolbelt_actions.pressed(ToolbeltAction::Show) {
            *toolbelt_visibility = Visibility::Visible;
            let open = ((toolbelt_actions.value(ToolbeltAction::Show) - PRESS_THRESHOLD)
                / (1.0 - PRESS_THRESHOLD)
                * 1.01)
                .min(1.0);
            wheel.radius = wheel.radius_start + (wheel.radius_end - wheel.radius_start) * open;
            wheel.spin = wheel.spin_start + (wheel.spin_end - wheel.spin_start) * open;
            wheel.scale = wheel.scale_start + (wheel.scale_end - wheel.scale_start) * open;
            if toolbelt_actions.just_pressed(ToolbeltAction::Show) {
                // ensure the tools are positioned nicely when the toolbelt is first opened
                distribute(toolbelt_kids, &mut tool_query, &mut tool_joint_query, &wheel);
                info!("Show toolbelt");
            }
        } else if toolbelt_actions.just_released(ToolbeltAction::Show) {
            *toolbelt_visibility = Visibility::Hidden;
            info!("Hide toolbelt");
        }
    }
}
