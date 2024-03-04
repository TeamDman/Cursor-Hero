use bevy::prelude::*;

use cursor_hero_toolbelt_types::toolbelt_types::ToolHelp;

pub fn tool_help_cleanup(
    mut commands: Commands,
    mut tool_help_query: Query<(Entity, &mut ToolHelp)>,
    time: Res<Time>,
) {
    for (tool_help_id, mut tool_help) in &mut tool_help_query {
        if tool_help.timer.tick(time.delta()).just_finished() {
            commands.entity(tool_help_id).despawn_recursive();
        }
    }
}
