use super::types::*;
use bevy::prelude::*;
use bevy_xpbd_2d::components::CollidingEntities;
use cursor_hero_bevy::NameOrEntityDisplay;
use cursor_hero_pointer::pointer_plugin::Pointer;

pub fn hover_detection(
    mut commands: Commands,
    pointer_query: Query<Entity, With<Pointer>>,
    hoverable_query: Query<
        (
            Entity,
            &CollidingEntities,
            &Visibility,
            Option<&Hovered>,
            Option<&Name>,
        ),
        Or<(With<Tool>, With<ToolHelpTrigger>)>,
    >,
    mut events: EventWriter<ToolHoveredEvent>,
) {
    let pointers = pointer_query.iter().collect::<Vec<_>>();
    for (hoverable_id, hoverable_touching, hoverable_visible, hoverable_hovered, hoverable_name) in hoverable_query.iter() {
        if hoverable_visible == Visibility::Hidden {
            continue;
        }
        if hoverable_touching.iter().any(|e| pointers.contains(e)) {
            if hoverable_hovered.is_none() {
                commands.entity(hoverable_id).insert(Hovered);
                events.send(ToolHoveredEvent::HoverStart(hoverable_id));
                debug!("Hovering over tool: {:?}", hoverable_name.name_or_entity(hoverable_id));
            }
        } else {
            if hoverable_hovered.is_some() {
                commands.entity(hoverable_id).remove::<Hovered>();
                events.send(ToolHoveredEvent::HoverEnd(hoverable_id));
                debug!("No longer hovering over tool: {:?}", hoverable_name.name_or_entity(hoverable_id));
            }
        }
    }
}
