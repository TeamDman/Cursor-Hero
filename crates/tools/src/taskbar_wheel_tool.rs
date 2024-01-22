use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_toolbelt::types::*;
use cursor_hero_winutils::ui_automation::get_taskbar;
use cursor_hero_winutils::ui_automation::TaskbarEntry;

pub struct TaskbarWheelToolPlugin;

impl Plugin for TaskbarWheelToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TaskbarWheelTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(Update, tick);
    }
}

#[derive(Component, Reflect)]
struct TaskbarWheelTool;

#[derive(Component, Reflect)]
struct TaskbarEntryTool(TaskbarEntry);

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for event in reader.read() {
        if let ToolbeltEvent::PopulateDefaultToolbelt {
            toolbelt_id,
            character_id,
        } = event
        {
            spawn_tool(
                Tool::create(file!(), "Swaps to taskbar tools".to_string(), &asset_server),
                event,
                &mut commands,
                *toolbelt_id,
                *character_id,
                &asset_server,
                TaskbarWheelTool,
                StartingState::Inactive,
            );
        }
        if let ToolbeltEvent::PopulateTaskbarToolbelt {
            toolbelt_id,
            character_id,
        } = event
        {
            let Ok(taskbar) = get_taskbar() else {
                continue;
            };
            for entry in taskbar.entries {
                spawn_tool(
                    Tool::create(file!(), "Swaps to taskbar tools".to_string(), &asset_server),
                    event,
                    &mut commands,
                    *toolbelt_id,
                    *character_id,
                    &asset_server,
                    TaskbarEntryTool(entry),
                    StartingState::Inactive,
                );
            }
        }
    }
}

fn tick(
    mut commands: Commands,
    tool_query: Query<&Parent, (Added<ActiveTool>, With<TaskbarWheelTool>)>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut toolbelt_events: EventWriter<ToolbeltEvent>,
) {
    for toolbelt_id in tool_query.iter() {
        let toolbelt_id = toolbelt_id.get();
        if let Ok(character_id) = toolbelt_query.get(toolbelt_id) {
            let character_id = character_id.get();
            commands.entity(toolbelt_id).despawn_descendants();
            toolbelt_events.send(ToolbeltEvent::PopulateTaskbarToolbelt {
                toolbelt_id: toolbelt_id,
                character_id: character_id,
            });
        }
    }
}
