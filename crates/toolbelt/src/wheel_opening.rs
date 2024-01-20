use super::types::*;
use bevy::prelude::*;
use cursor_hero_input::update_gamepad_settings::PRESS_THRESHOLD;
use leafwing_input_manager::action_state::ActionState;

#[allow(clippy::type_complexity)]
pub fn wheel_opening(
    mut toolbelt_query: Query<
        (Entity, &ActionState<ToolbeltAction>, &mut Wheel),
        (Without<Tool>, With<Toolbelt>),
    >,
    mut toolbelt_events: EventWriter<ToolbeltEvent>,
) {
    for (toolbelt_id, toolbelt_actions, mut wheel) in toolbelt_query.iter_mut() {
        if toolbelt_actions.pressed(ToolbeltAction::Show) {
            let open = ((toolbelt_actions.value(ToolbeltAction::Show) - PRESS_THRESHOLD)
                / (1.0 - PRESS_THRESHOLD)
                * 1.01)
                .min(1.0);
            wheel.radius = wheel.radius_start + (wheel.radius_end - wheel.radius_start) * open;
            wheel.spin = wheel.spin_start + (wheel.spin_end - wheel.spin_start) * open;
            wheel.scale = wheel.scale_start + (wheel.scale_end - wheel.scale_start) * open;
            wheel.alpha = wheel.alpha_start + (wheel.alpha_end - wheel.alpha_start) * open;
            if !wheel.open {
                toolbelt_events.send(ToolbeltEvent::Opened { toolbelt_id });
                wheel.open = true;
            }
        } else {
            if wheel.open {
                toolbelt_events.send(ToolbeltEvent::Closed { toolbelt_id });
                wheel.open = false;
            }
        }
    }
}
