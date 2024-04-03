use std::f32::consts::PI;

use bevy_xpbd_2d::components::Position;
use bevy_xpbd_2d::components::Rotation;
use bevy_xpbd_2d::PhysicsSet;
use cursor_hero_toolbelt_types::toolbelt_types::*;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use cursor_hero_input::update_gamepad_settings::PRESS_THRESHOLD;
use cursor_hero_cursor_types::prelude::*;

use itertools::Itertools;
use leafwing_input_manager::action_state::ActionState;

pub struct ToolbeltCircleLayoutPlugin;

impl Plugin for ToolbeltCircleLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_render_data);
        app.add_systems(Update, reset_reach);
        app.add_systems(
            PostUpdate,
            position_tools
                .after(PhysicsSet::Sync)
                .after(TransformSystem::TransformPropagate),
        );
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

#[allow(clippy::type_complexity)]
pub fn position_tools(
    toolbelt_query: Query<(Ref<Toolbelt>, &Children, &Parent), Without<Tool>>,
    character_query: Query<&GlobalTransform>,
    mut tool_query: Query<
        (&mut Transform, &mut Position, &mut Rotation, &Children),
        (With<Tool>, Without<ToolHelpTrigger>),
    >,
    mut tool_help_query: Query<
        (&mut Transform, &mut Position, &mut Rotation),
        (With<ToolHelpTrigger>, Without<Tool>),
    >,
) {
    for (toolbelt, toolbelt_kids, toolbelt_parent) in toolbelt_query.iter() {
        if !toolbelt.is_changed() {
            continue;
        }
        let ToolbeltLayout::Circle { wheel } = toolbelt.layout else {
            continue;
        };
        if let Ok(character_position) = character_query.get(**toolbelt_parent) {
            let tool_ids = toolbelt_kids
                .iter()
                .filter(|e| tool_query.contains(**e))
                .collect_vec();
            position_tools_helper(
                character_position,
                tool_ids,
                &mut tool_query,
                &mut tool_help_query,
                &wheel,
            );
        }
    }
}

#[allow(clippy::type_complexity)]
fn position_tools_helper(
    character_position: &GlobalTransform,
    tool_ids: Vec<&Entity>,
    tool_query: &mut Query<
        (&mut Transform, &mut Position, &mut Rotation, &Children),
        (With<Tool>, Without<ToolHelpTrigger>),
    >,
    tool_help_query: &mut Query<
        (&mut Transform, &mut Position, &mut Rotation),
        (With<ToolHelpTrigger>, Without<Tool>),
    >,
    wheel: &Wheel,
) {
    let count = tool_ids.len();
    for (i, tool_id) in tool_ids.iter().enumerate() {
        if let Ok((mut tool_transform, mut tool_position, mut tool_rotation, tool_children)) =
            tool_query.get_mut(**tool_id)
        {
            let angle = 2.0 * PI / (count as f32) * i as f32;
            let x = angle.cos();
            let y = angle.sin();
            tool_transform.scale = Vec2::splat(wheel.scale).extend(1.0);
            let character_position = character_position.translation().xy();
            tool_position.0 = character_position + Vec2::new(x * wheel.radius, y * wheel.radius);
            *tool_rotation = Rotation::from_degrees(wheel.spin);
            for tool_child in tool_children.iter() {
                if let Ok((
                    mut tool_help_transform,
                    mut tool_help_position,
                    mut tool_help_rotation,
                )) = tool_help_query.get_mut(*tool_child)
                {
                    tool_help_transform.scale = Vec2::splat(wheel.scale).extend(1.0);
                    tool_help_position.0 = tool_position.xy()
                        + Vec2::new(x * -wheel.radius * 0.5, y * -wheel.radius * 0.5);
                    *tool_help_rotation = Rotation::from_degrees(wheel.spin);
                }
            }
        }
    }
}
