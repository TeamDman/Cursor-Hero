use super::types::*;
use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn wheel_radius(
    toolbelts: Query<
        (Ref<CirclularDistributionProperties>, &Children),
        (With<Toolbelt>, Without<Tool>),
    >,
    mut tools: Query<&mut Transform, (With<Tool>, Without<Toolbelt>)>,
) {
    for (circle, children) in toolbelts.iter() {
        if circle.is_changed() {
            let count = children.iter().count();
            for (i, tool) in children.iter().enumerate() {
                let angle = 360.0 / (count as f32) * i as f32;
                let x = angle.to_radians().cos() * circle.radius;
                let y = angle.to_radians().sin() * circle.radius;
                if let Ok(mut tool_transform) = tools.get_mut(*tool) {
                    tool_transform.translation = Vec3::new(x, y, 200.0);
                }
            }
        }
    }
}
