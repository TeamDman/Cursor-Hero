use bevy::prelude::*;
use cursor_hero_{{crate_name_pascal}}_tool_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct {{crate_name_pascal}}ToolTickPlugin;

impl Plugin for {{crate_name_pascal}}ToolTickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<{{crate_name_pascal}}ToolAction>::default());
        app.add_systems(Update, tick.after(DampingSystemSet::Dampen));
    }
}

fn tick(
    time: Res<Time<Physics>>,
    tool_query: Query<(&ActionState<{{crate_name_pascal}}ToolAction>, &{{crate_name_pascal}}Tool), With<ActiveTool>>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for tool in tool_query.iter() {
        let (tool_actions, tool) = tool;
        if !tool_actions.pressed({{crate_name_pascal}}ToolAction::Use) {
            continue;
        }
        info!("{{crate_name_pascal}}Tool used!");
    }
}
