use bevy::prelude::*;
use cursor_hero_physics::damping_plugin::DampingSystemSet;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_movement_tool_types::prelude::*;
use leafwing_input_manager::prelude::*;

use bevy_xpbd_2d::math::*;
use bevy_xpbd_2d::prelude::*;

pub struct MovementToolTickPlugin;

impl Plugin for MovementToolTickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MovementToolAction>::default());
        app.add_systems(Update, handle_inputs.after(DampingSystemSet::Dampen));
    }
}

fn handle_inputs(
    time: Res<Time<Physics>>,
    tool_query: Query<(&ActionState<MovementToolAction>, &MovementTool, &Parent), With<ActiveTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut character_query: Query<&mut LinearVelocity, (With<Character>, Without<Camera>)>,
    mut camera_query: Query<&mut LinearVelocity, (With<Camera>, Without<Character>)>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for tool in tool_query.iter() {
        let (tool_actions, tool, tool_parent) = tool;
        if !tool_actions.pressed(MovementToolAction::Move) {
            continue;
        }
        let Ok(toolbelt_parent) = toolbelt_query.get(tool_parent.get()) else {
            continue;
        };
        let move_delta = delta_time
            * tool_actions
                .clamped_axis_pair(MovementToolAction::Move)
                .unwrap()
                .xy();
        match tool.target {
            MovementTarget::Character => {
                let Ok(character) = character_query.get_mut(toolbelt_parent.get()) else {
                    warn!("Character {:?} does not exist", toolbelt_parent);
                    continue;
                };
                let mut character_velocity = character;
                character_velocity.x += move_delta.x * tool.speed;
                character_velocity.y += move_delta.y * tool.speed;
            }
            MovementTarget::Camera(camera_id) => {
                let Ok(camera) = camera_query.get_mut(camera_id) else {
                    warn!("Camera {:?} does not exist", camera_id);
                    continue;
                };
                let mut camera_velocity = camera;
                camera_velocity.x += move_delta.x * tool.speed;
                camera_velocity.y += move_delta.y * tool.speed;
            }
        }
    }
}
