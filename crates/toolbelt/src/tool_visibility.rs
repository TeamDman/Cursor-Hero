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
    mut tool_query: Query<(Entity, &mut Transform, &mut Visibility, &mut Sprite), With<Tool>>,
) {
    for (toolbelt_actions, wheel, toolbelt_kids) in toolbelts.iter_mut() {
        if toolbelt_actions.just_pressed(ToolbeltAction::Show) {
            info!("Show toolbelt");
            for child_id in toolbelt_kids.iter() {
                if let Ok((_, _, mut tool_visibility, _)) = tool_query.get_mut(*child_id) {
                    *tool_visibility = Visibility::Visible;
                }
            }
        } else if toolbelt_actions.just_released(ToolbeltAction::Show) {
            info!("Hide toolbelt");
            for child_id in toolbelt_kids.iter() {
                if let Ok((_, _, mut tool_visibility, _)) = tool_query.get_mut(*child_id) {
                    *tool_visibility = Visibility::Hidden;
                }
            }
        }
        if wheel.open {
            for child_id in toolbelt_kids.iter() {
                if let Ok((_, _, _, mut tool_sprite)) = tool_query.get_mut(*child_id) {
                    tool_sprite.color = tool_sprite.color.with_a(wheel.alpha);
                }
            }
        }
    }
}
