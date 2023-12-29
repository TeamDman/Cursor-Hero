use bevy::prelude::*;
use bevy_xpbd_2d::components::LinearVelocity;
use super::types::*;

pub fn tool_hovered_tag_update_system(
    mut commands: Commands,
    toolbelts: Query<(&Visibility, &Children, &Parent), With<Toolbelt>>,
    follow: Query<&LinearVelocity>,
    tools: Query<(&Transform, Option<&ToolHoveredTag>)>,
    mut events: EventWriter<ToolHoveredEvent>,
) {
    for (t_vis, t_kids, t_parent) in toolbelts.iter() {
        if t_vis != &Visibility::Visible {
            continue;
        }
        if let Ok(follow_vel) = follow.get(t_parent.get()) {
            let mut closest = None;
            if follow_vel.x.abs() > 25.0 || follow_vel.y.abs() > 25.0 {
                // we want to find the toolbelt entry that is closest to the direction of the movement of the follow entity
                // find the angle between the follow entity and each toolbelt entry
                // find the angle of the direction of travel
                // find the tool with the smallest difference between the two angles
                let travel_angle =
                    normalize_angle(follow_vel.0.angle_between(Vec2::new(1.0, 0.0)));
                let mut closest_angle = std::f32::consts::PI; // Initialized to the max angle difference (180 degrees)

                for kid in t_kids.iter() {
                    if let Ok((kid_transform, _hovered_status)) = tools.get(*kid) {
                        let kid_angle = normalize_angle(
                            kid_transform
                                .translation
                                .xy()
                                .angle_between(Vec2::new(1.0, 0.0)),
                        );
                        let diff = angular_diff(kid_angle, travel_angle);

                        if diff < closest_angle {
                            closest = Some(*kid);
                            closest_angle = diff;
                        }
                    }
                }
            }
            // remove the follow tag from the unhovered tools
            for kid in t_kids.iter() {
                if Some(*kid) != closest {
                    if let Ok((_, hovered_status)) = tools.get(*kid) {
                        if hovered_status.is_some() {
                            commands.entity(*kid).remove::<ToolHoveredTag>();
                            events.send(ToolHoveredEvent::HoverEnd(*kid));
                        }
                    }
                }
            }
            if let Some(closest) = closest {
                // add the follow tag to the closest tool
                // if the closest tool already has the follow tag, do nothing
                if let Ok((_, hovered_status)) = tools.get(closest) {
                    if hovered_status.is_none() {
                        commands.entity(closest).insert(ToolHoveredTag);
                        events.send(ToolHoveredEvent::HoverStart(closest));
                    }
                }
            }
        }
    }
}

pub fn normalize_angle(angle: f32) -> f32 {
    let two_pi = std::f32::consts::PI * 2.0;
    (angle + two_pi) % two_pi
}

pub fn angular_diff(angle1: f32, angle2: f32) -> f32 {
    let diff = (angle1 - angle2).abs();
    if diff > std::f32::consts::PI {
        std::f32::consts::PI * 2.0 - diff
    } else {
        diff
    }
}
