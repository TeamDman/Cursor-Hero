use super::types::*;
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::prelude::*;
use cursor_hero_pointer::pointer_plugin::Pointer;
use itertools::Itertools;

/// This system places the tools in a circle around the toolbelt wearer.
/// It also adjusts the pointer radius to match the toolbelt radius.
#[allow(clippy::type_complexity)]
pub fn wheel_distribution(
    toolbelts: Query<(Ref<Wheel>, &Children, &Parent), (With<Toolbelt>, Without<Tool>)>,
    wearer_query: Query<&Children>,
    mut tool_query: Query<&mut Transform, (With<Tool>, Without<Toolbelt>)>,
    mut pointer_query: Query<&mut Pointer>,
) {
    for (wheel, tools, wearer) in toolbelts.iter() {
        if wheel.is_changed() {
            debug!("wheel changed: {:?}", wheel);
            // distribute the tools
            distribute(tools, &mut tool_query, &wheel);

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
    wheel: &Wheel,
) {
    let tools = toolbelt_children
        .iter()
        .filter(|e| tool_query.get(**e).is_ok())
        .collect_vec();
    let count = tools.len();
    for (i, tool) in tools.into_iter().enumerate() {
        let angle = 360.0 / (count as f32) * i as f32;
        let x = angle.to_radians().cos() * wheel.radius;
        let y = angle.to_radians().sin() * wheel.radius;
        let tool_transform = &mut tool_query.get_mut(*tool).unwrap();
        tool_transform.translation.x = x;
        tool_transform.translation.y = y;
        tool_transform.rotation = Quat::from_rotation_z((wheel.spin).to_radians());
        tool_transform.scale = Vec2::splat(wheel.scale).extend(1.0);
    }
}
