use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::utils::HashSet;
use bevy_xpbd_2d::components::Position;
use bevy_xpbd_2d::components::Rotation;
use bevy_xpbd_2d::PhysicsSet;
use cursor_hero_math::prelude::Corner;
use cursor_hero_toolbelt_types::toolbelt_types::Tool;
use cursor_hero_toolbelt_types::toolbelt_types::ToolHelpTrigger;
use cursor_hero_toolbelt_types::toolbelt_types::Toolbelt;
use cursor_hero_toolbelt_types::toolbelt_types::ToolbeltLayout;
use cursor_hero_window_position_types::prelude::WindowPositionTool;
use cursor_hero_window_position_types::window_position_types::HostWindowPosition;
use itertools::Itertools;

pub struct ToolbeltTaskbarLayoutPlugin;

impl Plugin for ToolbeltTaskbarLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            position_tools
                .after(PhysicsSet::Sync)
                .after(TransformSystem::TransformPropagate),
        );
    }
}

#[allow(clippy::type_complexity)]
pub fn position_tools(
    toolbelt_query: Query<(Ref<Toolbelt>, &Children, &Parent), Without<Tool>>,
    character_query: Query<&GlobalTransform>,
    mut tool_query: Query<
        (
            Entity,
            Option<&WindowPositionTool>,
            &mut Transform,
            &mut Position,
            &mut Rotation,
            &Children,
        ),
        (With<Tool>, Without<ToolHelpTrigger>),
    >,
    mut tool_help_query: Query<
        (&mut Transform, &mut Position, &mut Rotation),
        (With<ToolHelpTrigger>, Without<Tool>),
    >,
) {
    for toolbelt in toolbelt_query.iter() {
        let (toolbelt, toolbelt_kids, toolbelt_parent) = toolbelt;
        if !toolbelt.is_changed() {
            continue;
        }
        let ToolbeltLayout::Taskbar { wheel, .. } = toolbelt.layout else {
            continue;
        };
        if let Ok(character_position) = character_query.get(**toolbelt_parent) {
            let sorted_window_tools = toolbelt_kids
                .iter()
                .filter(|e| tool_query.contains(**e))
                .filter_map(|e| tool_query.get(*e).ok())
                .filter_map(|(entity, window_position_tool, ..)| {
                    window_position_tool.map(|window_position_tool| (entity, window_position_tool))
                })
                .sorted_by_key(|(_, window_position_tool)| {
                    match window_position_tool.window_position {
                        HostWindowPosition::Corner {
                            ref corner,
                            monitor,
                        } => {
                            monitor * 100
                                + match corner {
                                    Corner::TopLeft => 0,
                                    Corner::TopRight => 1,
                                    Corner::BottomLeft => 2,
                                    Corner::BottomRight => 3,
                                }
                        }
                        HostWindowPosition::Fullscreen { monitor } => 100000 + monitor,
                    }
                })
                .map(|(entity, _)| entity)
                .collect_vec();
            let window_tools = sorted_window_tools.iter().collect::<HashSet<_>>();
            let remaining_tools = toolbelt_kids
                .iter()
                .filter(|e| tool_query.contains(**e))
                .filter(|e| !window_tools.contains(e))
                .collect_vec();

            let count = remaining_tools.len();
            for (i, tool_id) in remaining_tools.iter().enumerate() {
                let Ok(tool) = tool_query.get_mut(**tool_id) else {
                    continue;
                };

                let (
                    _tool_id,
                    _position_tool,
                    mut tool_transform,
                    mut tool_position,
                    mut tool_rotation,
                    tool_children,
                ) = tool;
                let angle = 2.0 * PI / (count as f32) * i as f32;
                let x = angle.cos();
                let y = angle.sin();
                tool_transform.scale = Vec2::splat(wheel.scale).extend(1.0);
                let character_position = character_position.translation().xy();
                tool_position.0 =
                    character_position + Vec2::new(x * wheel.radius, y * wheel.radius);
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
}
