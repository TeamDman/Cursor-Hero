use super::types::*;
use bevy::prelude::*;
use bevy_xpbd_2d::components::CollidingEntities;
use cursor_hero_pointer::pointer_plugin::Pointer;

pub fn hover_tag(
    mut commands: Commands,
    pointer_query: Query<Entity, With<Pointer>>,
    tool_query: Query<(Entity, &CollidingEntities, &InheritedVisibility, Option<&HoveredTool>), With<Tool>>,
    mut events: EventWriter<ToolHoveredEvent>,
) {
    let pointers = pointer_query.iter().collect::<Vec<_>>();
    for (tool_id, tool_touching, tool_visible, tool_hovered) in tool_query.iter() {
        if !tool_visible.get() {
            continue;
        }
        if tool_touching.iter().any(|e| pointers.contains(e)) {
            if tool_hovered.is_none() {
                commands.entity(tool_id).insert(HoveredTool);
                events.send(ToolHoveredEvent::HoverStart(tool_id));
            }
        } else {
            if tool_hovered.is_some() {
                commands.entity(tool_id).remove::<HoveredTool>();
                events.send(ToolHoveredEvent::HoverEnd(tool_id));
            }
        }
    }
}