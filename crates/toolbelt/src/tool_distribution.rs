use super::types::*;
use bevy::ecs::query::QueryManyIter;
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_xpbd_2d::constraints::FixedJoint;
use cursor_hero_pointer::pointer_plugin::Pointer;
use itertools::Itertools;

#[allow(clippy::type_complexity)]
pub fn tool_distribution(
    toolbelts: Query<(Ref<Wheel>, &Children, &Parent), (With<Toolbelt>, Without<Tool>)>,
    wearer_query: Query<&Children>,
    mut tool_query: Query<(Entity, &mut Transform, &mut Visibility), With<Tool>>,
    mut tool_joint_query: Query<&mut FixedJoint, With<ToolJoint>>,
    mut pointer_query: Query<&mut Pointer>,
) {
    for (wheel, toolbelt_kids, toolbelt_parent) in toolbelts.iter() {
        if wheel.is_changed() {
            {
                let tool_joint_ids = toolbelt_kids
                    .iter()
                    .filter(|e| tool_joint_query.get(**e).is_ok())
                    .collect_vec();
                update_joints(tool_joint_ids, &mut tool_joint_query, &wheel);
            }
            
            // adjust the pointer radius
            if let Ok(wearer) = wearer_query.get(**toolbelt_parent) {
                for kid in wearer.iter() {
                    if let Ok(mut pointer) = pointer_query.get_mut(*kid) {
                        pointer.reach = wheel.radius;
                    }
                }
            }
        }
    }
}

fn update_joints(
    tool_joint_ids: Vec<&Entity>,
    tool_joint_query: &mut Query<&mut FixedJoint, With<ToolJoint>>,
    wheel: &Wheel,
) {
    let count = tool_joint_ids.len();
    debug!("update_joints: count: {}", count);
    for (i, tool_joint_id) in tool_joint_ids.iter().enumerate() {
        if let Ok(mut tool_joint) = tool_joint_query.get_mut(**tool_joint_id) {
            let angle = 360.0 / (count as f32) * i as f32;
            let x = angle.to_radians().cos() * wheel.radius;
            let y = angle.to_radians().sin() * wheel.radius;
            tool_joint.local_anchor1 = Vec2::new(x, y);
        }

        // let tool_transform = &mut tool_transform_query.get_mut(*tool).unwrap();
        // tool_transform.translation.x = x;
        // tool_transform.translation.y = y;
        // tool_transform.rotation = Quat::from_rotation_z((wheel.spin).to_radians());
        // tool_transform.scale = Vec2::splat(wheel.scale).extend(1.0);
    }
}
