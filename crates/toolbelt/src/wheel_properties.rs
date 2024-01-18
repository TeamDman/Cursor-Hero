use super::types::*;
use bevy::prelude::*;
use cursor_hero_input::update_gamepad_settings::PRESS_THRESHOLD;
use leafwing_input_manager::action_state::ActionState;

pub fn wheel_properties(
    mut toolbelts: Query<
        (&ActionState<ToolbeltAction>, &mut Wheel, &Children),
        (Without<Tool>, With<Toolbelt>),
    >,
) {
    for (toolbelt_actions, mut wheel, toolbelt_kids) in toolbelts.iter_mut() {
        if toolbelt_actions.pressed(ToolbeltAction::Show) {
            let open = ((toolbelt_actions.value(ToolbeltAction::Show) - PRESS_THRESHOLD)
                / (1.0 - PRESS_THRESHOLD)
                * 1.01)
                .min(1.0);
            wheel.radius = wheel.radius_start + (wheel.radius_end - wheel.radius_start) * open;
            wheel.spin = wheel.spin_start + (wheel.spin_end - wheel.spin_start) * open;
            wheel.scale = wheel.scale_start + (wheel.scale_end - wheel.scale_start) * open;
            wheel.alpha = wheel.alpha_start + (wheel.alpha_end - wheel.alpha_start) * open;
            wheel.open = true;
        } else {
            wheel.open = false;
        }
    }
}
