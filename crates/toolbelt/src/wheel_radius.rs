use super::types::*;
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::prelude::*;
use cursor_hero_pointer::pointer_plugin::Pointer;
use itertools::Itertools;

/// This system places the tools in a circle around the toolbelt wearer.
/// It also adjusts the pointer radius to match the toolbelt radius.
#[allow(clippy::type_complexity)]
pub fn wheel_radius(
    toolbelts: Query<(Ref<Wheel>, &Children, &Parent), (With<Toolbelt>, Without<Tool>)>,
    wearer_query: Query<&Children>,
    mut tool_query: Query<&mut Transform, (With<Tool>, Without<Toolbelt>)>,
    mut pointer_query: Query<&mut Pointer>,
) {
    for (wheel, tools, wearer) in toolbelts.iter() {
        if wheel.is_changed() {
            // distribute the tools
            distribute(tools, &mut tool_query, wheel.radius);

            // adjust the pointer radius
            if let Ok(wearer) = wearer_query.get(**wearer) {
                for kid in wearer.iter() {
                    if let Ok(mut pointer) = pointer_query.get_mut(*kid) {
                        pointer.reach = wheel.radius;
                    }
                }
            }
        }
    }
}

pub fn distribute<T: ReadOnlyWorldQuery>(
    toolbelt_children: &Children,
    tool_query: &mut Query<&mut Transform, T>,
    radius: f32,
) {
    let tools = toolbelt_children
        .iter()
        .filter(|e| tool_query.get(**e).is_ok())
        .collect_vec();
    let count = tools.len();
    for (i, tool) in tools.into_iter().enumerate() {
        let angle = 360.0 / (count as f32) * i as f32;
        let x = angle.to_radians().cos() * radius;
        let y = angle.to_radians().sin() * radius;
        let tool_pos = &mut tool_query.get_mut(*tool).unwrap().translation;
        tool_pos.x = x;
        tool_pos.y = y;
    }
}
