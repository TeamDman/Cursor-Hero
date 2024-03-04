use cursor_hero_toolbelt_types::toolbelt_types::*;

use bevy::prelude::*;
use cursor_hero_input::update_gamepad_settings::PRESS_THRESHOLD;
use cursor_hero_pointer_types::prelude::*;

use leafwing_input_manager::action_state::ActionState;

pub struct ToolbeltCircleLayoutPlugin;

impl Plugin for ToolbeltCircleLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_render_data);
        app.add_systems(Update, reset_reach);
    }
}

#[allow(clippy::type_complexity)]
pub fn update_render_data(
    mut toolbelt_query: Query<
        (
            &mut Toolbelt,
            &ActionState<ToolbeltAction>,
            &Parent,
            &Children,
        ),
        Without<Tool>,
    >,
    tool_query: Query<Entity, With<Tool>>,
    mut pointer_reach_events: EventWriter<PointerReachEvent>,
) {
    for toolbelt in toolbelt_query.iter_mut() {
        let (mut toolbelt, toolbelt_actions, toolbelt_parent, toolbelt_children) = toolbelt;
        if !toolbelt.open {
            continue;
        }
        let ToolbeltLayout::Circle { wheel, .. } = &mut toolbelt.layout else {
            continue;
        };
        let tool_count = toolbelt_children
            .iter()
            .filter(|e| tool_query.get(**e).is_ok())
            .count();
        let open = ((toolbelt_actions.value(ToolbeltAction::Show) - PRESS_THRESHOLD)
            / (1.0 - PRESS_THRESHOLD)
            * 1.01)
            .min(1.0);
        wheel.radius = wheel.radius_start
            + ((wheel.radius_end
                + wheel.radius_end_bonus_per_tool_after_8
                    * (tool_count as isize - 8).max(0) as f32)
                - wheel.radius_start)
                * open;
        wheel.spin = wheel.spin_start + (wheel.spin_end - wheel.spin_start) * open;
        wheel.scale = wheel.scale_start + (wheel.scale_end - wheel.scale_start) * open;
        wheel.alpha = wheel.alpha_start + (wheel.alpha_end - wheel.alpha_start) * open;
        pointer_reach_events.send(PointerReachEvent::SetCharacter {
            character_id: toolbelt_parent.get(),
            reach: wheel.radius,
        });
    }
}

fn reset_reach(
    mut pointer_reach_events: EventWriter<PointerReachEvent>,
    mut toolbelt_opening_events: EventReader<ToolbeltOpeningEvent>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
) {
    for event in toolbelt_opening_events.read() {
        let ToolbeltOpeningEvent::Closed { toolbelt_id } = event else {
            continue;
        };
        let Ok(toolbelt) = toolbelt_query.get(*toolbelt_id) else {
            continue;
        };
        let character_id = toolbelt.get();
        pointer_reach_events.send(PointerReachEvent::ResetCharacter { character_id });
    }
}
