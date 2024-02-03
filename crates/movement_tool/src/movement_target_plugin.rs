use bevy::prelude::*;
use cursor_hero_movement_tool_types::prelude::*;

pub struct MovementTargetPlugin;

impl Plugin for MovementTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_set_movement_events);
    }
}


fn handle_set_movement_events(
    mut movement_target_events: EventReader<MovementTargetEvent>,
    mut tool_query: Query<&mut MovementTool>,
) {
    for event in movement_target_events.read() {
        match event {
            MovementTargetEvent::SetTarget { tool_id, target } => {
                let Ok(mut tool) = tool_query.get_mut(*tool_id) else {
                    warn!("Tool {:?} does not exist", tool_id);
                    continue;
                };
                tool.target = *target;
            }
        }
    }
}