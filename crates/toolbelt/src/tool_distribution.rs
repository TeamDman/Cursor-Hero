use super::types::*;
use bevy::prelude::*;
use bevy_xpbd_2d::math::PI;
use bevy_xpbd_2d::prelude::*;
use itertools::Itertools;

#[allow(clippy::type_complexity)]
pub fn tool_distribution(
    toolbelt_query: Query<(Ref<Wheel>, &Children, &Parent), (With<Toolbelt>, Without<Tool>)>,
    character_query: Query<&GlobalTransform>,
    mut tool_query: Query<(&mut Transform, &mut Position, &mut Rotation), With<Tool>>,
) {
    for (wheel, toolbelt_kids, toolbelt_parent) in toolbelt_query.iter() {
        if !wheel.is_changed() {
            continue;
        }
        if let Ok(character_position) = character_query.get(**toolbelt_parent) {
            let tool_ids = toolbelt_kids
                .iter()
                .filter(|e| tool_query.get(**e).is_ok())
                .collect_vec();
            update_joints(character_position, tool_ids, &mut tool_query, &wheel);
        }
    }
}

fn update_joints(
    character_position: &GlobalTransform,
    tool_ids: Vec<&Entity>,
    tool_query: &mut Query<(&mut Transform, &mut Position, &mut Rotation), With<Tool>>,
    wheel: &Wheel,
) {
    let count = tool_ids.len();
    for (i, tool_id) in tool_ids.iter().sorted().enumerate() {
        if let Ok((mut tool_transform, mut tool_position, mut tool_rotation)) =
            tool_query.get_mut(**tool_id)
        {
            let angle = 2.0 * PI / (count as f32) * i as f32;
            let x = angle.cos() * wheel.radius;
            let y = angle.sin() * wheel.radius;
            tool_transform.scale = Vec2::splat(wheel.scale).extend(1.0);
            let character_position = character_position.translation().xy();
            tool_position.0 = character_position + Vec2::new(x, y);
            *tool_rotation = Rotation::from_degrees(wheel.spin);
        }
    }
}
