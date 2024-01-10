use super::types::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

pub fn wheel_visibility(
    mut toolbelts: Query<
        (
            &ActionState<ToolbeltAction>,
            &mut Visibility,
            &mut CirclularDistributionProperties,
            &Children,
        ),
        Without<Tool>,
    >,
    mut tools: Query<&mut Transform, With<Tool>>,
) {
    for (t_act, mut t_vis, mut t_circle, t_kids) in toolbelts.iter_mut() {
        if t_act.pressed(ToolbeltAction::Show) {
            *t_vis = Visibility::Visible;
            let open = t_act.value(ToolbeltAction::Show);
            // debug!("open: {}", open);
            t_circle.radius =
                t_circle.min_radius + (t_circle.max_radius - t_circle.min_radius) * open;
            if t_act.just_pressed(ToolbeltAction::Show) {
                // when first become visible, reset the tool positions
                {
                    let count = t_kids.iter().count();
                    info!("Applying circle layout to {} tools", count);
                    for (i, tool_id) in t_kids.iter().enumerate() {
                        let angle = 360.0 / (count as f32) * i as f32;
                        let x = angle.to_radians().cos() * t_circle.radius;
                        let y = angle.to_radians().sin() * t_circle.radius;
                        let mut transform = tools.get_mut(*tool_id).unwrap();
                        transform.translation = Vec3::new(x, y, 200.0);
                    }
                }
                info!("Show toolbelt");
            }
        } else if t_act.just_released(ToolbeltAction::Show) {
            *t_vis = Visibility::Hidden;
            info!("Hide toolbelt");
        }
    }
}
