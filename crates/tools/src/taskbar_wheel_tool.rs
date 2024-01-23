use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_xpbd_2d::components::Position;
use cursor_hero_glam::NegativeY;
use cursor_hero_screen::get_image::get_image;
use cursor_hero_screen::get_image::ScreensToImageParam;
use cursor_hero_toolbelt::types::*;
use cursor_hero_winutils::ui_automation::get_element_at;
use cursor_hero_winutils::ui_automation::get_taskbar;
use cursor_hero_winutils::ui_automation::TaskbarEntry;
use cursor_hero_winutils::win_window::focus_window;

pub struct TaskbarWheelToolPlugin;

impl Plugin for TaskbarWheelToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TaskbarWheelTool>()
            .register_type::<TaskbarEntryTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(Update, tick_wheel_switcher)
            .add_systems(Update, tick_taskbar_switcher);
    }
}

#[derive(Component, Reflect)]
struct TaskbarWheelTool;

#[derive(Component, Reflect)]
struct TaskbarEntryTool {
    entry: TaskbarEntry,
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
    access: ScreensToImageParam,
) {
    for event in reader.read() {
        if let ToolbeltPopulateEvent::Default {
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
                None,
            );
        }
        if let ToolbeltPopulateEvent::Taskbar {
            toolbelt_id,
            character_id,
        } = event
        {
            let Ok(taskbar) = get_taskbar() else {
                continue;
            };
            // TODO: ensure tool wheel getting populated according to taskbar
            for entry in taskbar.entries {
                let Ok(image) = get_image(entry.bounds.as_rect(), &access) else {
                    warn!("Failed to get image for {:?}", &entry);
                    continue;
                };
                let texture = asset_server.add(image);
                let size = Some(entry.bounds.size().as_vec2());
                spawn_tool(
                    Tool::new(
                        entry.name.clone(),
                        "Swaps to taskbar tools".to_string(),
                        HashMap::default(),
                        texture,
                    ),
                    event,
                    &mut commands,
                    *toolbelt_id,
                    *character_id,
                    &asset_server,
                    TaskbarEntryTool { entry },
                    StartingState::Inactive,
                    size,
                );
            }
        }
    }
}

fn tick_wheel_switcher(
    mut commands: Commands,
    tool_query: Query<&Parent, (Added<ActiveTool>, With<TaskbarWheelTool>)>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut toolbelt_events: EventWriter<ToolbeltPopulateEvent>,
) {
    for toolbelt_id in tool_query.iter() {
        let toolbelt_id = toolbelt_id.get();
        if let Ok(character_id) = toolbelt_query.get(toolbelt_id) {
            let character_id = character_id.get();
            commands.entity(toolbelt_id).despawn_descendants();
            toolbelt_events.send(ToolbeltPopulateEvent::Taskbar {
                toolbelt_id: toolbelt_id,
                character_id: character_id,
            });
        }
    }
}

fn tick_taskbar_switcher(
    mut commands: Commands,
    tool_query: Query<(&Parent, &TaskbarEntryTool), Added<ActiveTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut character_query: Query<&mut Position>,
    mut toolbelt_events: EventWriter<ToolbeltPopulateEvent>,
) {
    for (toolbelt_id, tool) in tool_query.iter() {
        let toolbelt_id = toolbelt_id.get();
        if let Ok(character_id) = toolbelt_query.get(toolbelt_id) {
            let character_id = character_id.get();
            commands.entity(toolbelt_id).despawn_descendants();
            toolbelt_events.send(ToolbeltPopulateEvent::Default {
                toolbelt_id: toolbelt_id,
                character_id: character_id,
            });
            if let Ok(mut position) = character_query.get_mut(character_id) {
                position.0 = tool.entry.bounds.center().as_vec2().neg_y();
            }
        }
    }
}
