use bevy::prelude::*;
use bevy_xpbd_2d::components::AngularVelocity;
use bevy_xpbd_2d::components::Rotation;
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
        app.add_systems(Update, keep_upright);
    }
}

#[allow(clippy::type_complexity)]
fn agent_tool_movement(
    character_query: Query<(&Children, &Transform), (With<Character>, With<Agent>)>,
    toolbelt_query: Query<&Children, With<Toolbelt>>,
    mut tool_query: Query<&mut ActionState<MovementToolAction>>,
    time: Res<Time>,
) {
    for character in character_query.iter() {
        let (character_children, character_transform) = character;
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
                let center = Vec2::new(1920.0, -1080.0) / 2.0;
                // walk in a circle around the center
                let desired_position = center + Vec2::from_angle(time.elapsed_seconds()) * 100.0;
                let direction = desired_position - character_transform.translation.xy();
                data.axis_pair = Some(DualAxisData::from_xy(direction.clamp_length_max(1.0)));
                tool.press(MovementToolAction::Move);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn keep_upright(
    mut character_query: Query<(&Rotation, &mut AngularVelocity), (With<Character>, With<Agent>)>,
) {
    for (rotation, mut angular_velocity) in character_query.iter_mut() {
        *angular_velocity = AngularVelocity(rotation.sin() * -1.0);
    }
}
