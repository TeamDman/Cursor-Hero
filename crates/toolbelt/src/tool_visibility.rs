use cursor_hero_toolbelt_types::toolbelt_types::*;

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

#[allow(clippy::type_complexity)]
pub fn tool_visibility(
    mut toolbelts: Query<
        (&ActionState<ToolbeltAction>, &mut Wheel, &Children),
        (Without<Tool>, With<Toolbelt>),
    >,
    mut tool_query: Query<(Entity, &mut Transform, &mut Visibility, &mut Sprite), With<Tool>>,
) {
    for (toolbelt_actions, wheel, toolbelt_kids) in toolbelts.iter_mut() {
        if toolbelt_actions.just_pressed(ToolbeltAction::Show) {
            debug!("Updating toolbelt visibility => visible");
            for child_id in toolbelt_kids.iter() {
                if let Ok((_, _, mut tool_visibility, _)) = tool_query.get_mut(*child_id) {
                    *tool_visibility = Visibility::Visible;
                }
            }
        } else if toolbelt_actions.just_released(ToolbeltAction::Show) {
            debug!("Updating toolbelt visibility => hidden");
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
