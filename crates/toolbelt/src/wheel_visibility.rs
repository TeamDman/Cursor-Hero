use crate::wheel_radius::distribute;

use super::types::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

pub fn wheel_visibility(
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
) {
    for (t_act, mut t_vis, mut t_circle, t_kids) in toolbelts.iter_mut() {
        if t_act.pressed(ToolbeltAction::Show) {
            *t_vis = Visibility::Visible;
            let open = t_act.value(ToolbeltAction::Show);
            t_circle.radius =
                t_circle.min_radius + (t_circle.max_radius - t_circle.min_radius) * open;
            if t_act.just_pressed(ToolbeltAction::Show) {
                // ensure the tools are positioned nicely when the toolbelt is first opened
                distribute(t_kids, &mut tool_query, t_circle.radius);
                info!("Show toolbelt");
            }
        } else if t_act.just_released(ToolbeltAction::Show) {
            *t_vis = Visibility::Hidden;
            info!("Hide toolbelt");
        }
    }
}
