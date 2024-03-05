use bevy::prelude::*;
use cursor_hero_WindowSwap_tool_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct WindowSwapToolTickPlugin;

impl Plugin for WindowSwapToolTickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<WindowSwapToolAction>::default());
        app.add_systems(Update, tick);
    }
}

fn tick(
    tool_query: Query<(&ActionState<WindowSwapToolAction>, &WindowSwapTool), With<ActiveTool>>,
) {
    for tool in tool_query.iter() {
        let (tool_actions, tool) = tool;
        if !tool_actions.pressed(WindowSwapToolAction::Use) {
            continue;
        }
        info!("WindowSwapTool used!");
    }
}
