use bevy::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_math::prelude::Lerp;
use cursor_hero_movement_tool_types::prelude::*;
use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintEvent;
use cursor_hero_toolbelt_types::prelude::*;
use itertools::Itertools;
pub struct MovementSprintPlugin;

impl Plugin for MovementSprintPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_sprint_events);
    }
}

fn handle_sprint_events(
    mut sprint_events: EventReader<SprintEvent>,
    character_query: Query<&Children, With<Character>>,
    toolbelt_query: Query<&Children, With<Toolbelt>>,
    mut tool_query: Query<&mut MovementTool>,
) {
    for event in sprint_events.read() {
        let character_id = match event {
            SprintEvent::Active { character_id, .. } => character_id,
            SprintEvent::Stop { character_id } => character_id,
        };
        let Ok(character) = character_query.get(*character_id) else {
            warn!("Character {:?} does not exist", character_id);
            continue;
        };
        let character_kids = character;

        let tool_ids = character_kids
            .iter()
            .filter_map(|kid| toolbelt_query.get(*kid).ok())
            .flat_map(|toolbelt| toolbelt.iter())
            .filter(|kid| tool_query.contains(**kid))
            .cloned()
            .collect_vec();

        match event {
            SprintEvent::Active { throttle, .. } => {
                let mut iter = tool_query.iter_many_mut(&tool_ids);
                while let Some(mut tool) = iter.fetch_next() {
                    tool.speed = (tool.default_speed, tool.sprint_speed).lerp(*throttle);
                }
            }
            SprintEvent::Stop { .. } => {
                let mut iter = tool_query.iter_many_mut(&tool_ids);
                while let Some(mut tool) = iter.fetch_next() {
                    tool.speed = tool.default_speed;
                }
            }
        }
    }
}
