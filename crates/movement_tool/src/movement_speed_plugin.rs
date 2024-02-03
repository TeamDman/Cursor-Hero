use bevy::prelude::*;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_movement_tool_types::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;

pub struct MovementSpeedPlugin;

impl Plugin for MovementSpeedPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_systems(OnEnter(ActiveInput::MouseKeyboard), set_mnk_speed);
        app.add_systems(OnEnter(ActiveInput::Gamepad), set_gamepad_speed);
    }
}


fn set_mnk_speed(
    mut tool_query: Query<(&mut MovementTool, &Parent), With<MovementTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    character_query: Query<(), With<MainCharacter>>,
) {
    for tool in tool_query.iter_mut() {
        let (mut tool, tool_parent) = tool;
        let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
            continue;
        };
        if character_query.get(toolbelt.get()).is_err() {
            continue;
        }
        *tool = MovementTool {
            target: tool.target,
            ..MovementTool::default_mnk()
        };
    }
}

fn set_gamepad_speed(
    mut tool_query: Query<(&mut MovementTool, &Parent), With<MovementTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    character_query: Query<(), With<MainCharacter>>,
) {
    for tool in tool_query.iter_mut() {
        let (mut tool, tool_parent) = tool;
        let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
            continue;
        };
        if character_query.get(toolbelt.get()).is_err() {
            continue;
        }
        *tool = MovementTool {
            target: tool.target,
            ..MovementTool::default_gamepad()
        };
    }
}
