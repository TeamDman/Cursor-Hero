use bevy::prelude::*;
use cursor_hero_explorer_tool_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ExplorerToolTickPlugin;

impl Plugin for ExplorerToolTickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ExplorerToolAction>::default());
        app.add_systems(Update, handle_toggle);
    }
}

fn handle_toggle(
    mut commands: Commands,
    tool_query: Query<&ExplorerTool>,
    mut tool_events: EventReader<ToolActivationEvent>,
) {
    for event in tool_events.read() {
        match event {
            ToolActivationEvent::Activate(tool_id)=> {
                let Ok(tool) = tool_query.get(*tool_id) else {
                    warn!("ExplorerTool with id {tool_id:?} not found.");
                    continue;
                };
                info!("ExplorerTool activated.");
            }
            ToolActivationEvent::Deactivate(tool_id) =>{
                let Ok(tool) = tool_query.get(*tool_id) else {
                    warn!("ExplorerTool with id {tool_id:?} not found.");
                    continue;
                };
                info!("ExplorerTool deactivated.");
            }
            _ => {}
        }
    }
}
