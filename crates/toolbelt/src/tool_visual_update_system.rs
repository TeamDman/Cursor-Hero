use super::types::*;
use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn tool_visual_update_system(
    mut query: Query<(&mut Sprite, Option<&ToolHoveredTag>, Option<&ToolActiveTag>), With<Tool>>,
) {
    // when a tool is updated, update the visuals for the tool
    // we read the events just to know when a change happens, we use the query optionals to determine desired state
    for (mut sprite, hovered, active) in query.iter_mut() {
        if let Some(_hovered) = hovered {
            if let Some(_active) = active {
                sprite.color = Color::rgb(0.0, 0.5, 0.5);
            } else {
                sprite.color = Color::rgb(0.5, 0.0, 0.5);
            }
        } else if let Some(_active) = active {
            sprite.color = Color::rgb(0.0, 0.5, 0.0);
        } else {
            sprite.color = Color::WHITE;
        }
    }
}
