use bevy::prelude::*;
use cursor_hero_agent_types::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_movement_tool_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::prelude::*;

pub struct AgentMovementPlugin;

impl Plugin for AgentMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, agent_tool_movement);
    }
}
fn agent_tool_movement(
    character_query: Query<&Children, (With<Character>, With<Agent>)>,
    toolbelt_query: Query<&Children, With<Toolbelt>>,
    mut tool_query: Query<&mut ActionState<MovementToolAction>>,
) {
    for character in character_query.iter() {
        let character_children = character;
        for character_child_id in character_children.iter() {
            let Ok(toolbelt) = toolbelt_query.get(*character_child_id) else {
                continue;
            };
            let toolbelt_children = toolbelt;
            for tool in toolbelt_children.iter() {
                let Ok(mut tool) = tool_query.get_mut(*tool) else {
                    continue;
                };
                let data = tool.action_data_mut(MovementToolAction::Move);
                data.axis_pair = Some(DualAxisData::from_xy(Vec2::new(1.0, 0.0)));
                tool.press(MovementToolAction::Move);
            }
        }
    }
}
