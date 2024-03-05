use bevy::prelude::*;
use bevy_xpbd_2d::components::Position;
use cursor_hero_bevy::prelude::NegativeYVec2;
use cursor_hero_screen::get_image::get_image;
use cursor_hero_screen::get_image::ScreensToImageParam;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::prelude::*;
use cursor_hero_ui_automation::prelude::find_element_at;
use cursor_hero_ui_automation::prelude::get_taskbar;
use cursor_hero_ui_automation::prelude::TaskbarEntry;

pub struct TaskbarWheelToolPlugin;

impl Plugin for TaskbarWheelToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TaskbarWheelTool>();
        app.register_type::<TaskbarEntryTool>();
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, tick_wheel_switcher);
        app.add_systems(Update, tick_taskbar_switcher);
    }
}

#[derive(Component, Reflect, Default)]
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
        match event.loadout {
            ToolbeltLoadout::Default => {
                ToolSpawnConfig::<TaskbarWheelTool, NoInputs>::new(
                    TaskbarWheelTool,
                    event.id,
                    event,
                )
                .with_src_path(file!().into())
                .guess_name(file!())
                .guess_image(file!(), &asset_server, "png")
                .with_description("Swaps to taskbar tools")
                .with_starting_state(StartingState::Inactive)
                .spawn(&mut commands);
            }
            ToolbeltLoadout::Taskbar => {
                let Ok(taskbar) = get_taskbar() else {
                    continue;
                };
                for entry in taskbar.entries {
                    let Ok(image) = get_image(entry.bounds.as_rect(), &access) else {
                        warn!("Failed to get image for {:?}", &entry);
                        continue;
                    };
                    ToolSpawnConfig::<TaskbarEntryTool, NoInputs>::new(
                        TaskbarEntryTool {
                            entry: entry.clone(),
                        },
                        event.id,
                        event,
                    )
                    .with_src_path(file!().into())
                    .with_name(entry.name)
                    .with_description("Swaps to taskbar tools")
                    .with_image(asset_server.add(image))
                    .with_size(entry.bounds.size().as_vec2())
                    .with_starting_state(StartingState::Inactive)
                    .spawn(&mut commands);
                }
            }
            _ => {}
        }
    }
}

fn tick_wheel_switcher(
    mut commands: Commands,
    tool_query: Query<&Parent, (Added<ActiveTool>, With<TaskbarWheelTool>)>,
    mut toolbelt_events: EventWriter<ToolbeltPopulateEvent>,
) {
    for toolbelt_id in tool_query.iter() {
        let toolbelt_id = toolbelt_id.get();
        commands.entity(toolbelt_id).despawn_descendants();
        toolbelt_events.send(ToolbeltPopulateEvent {
            id: toolbelt_id,
            loadout: ToolbeltLoadout::Taskbar,
        });
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
            info!("Switching toolbelt {:?} to default tools", toolbelt_id);
            let character_id = character_id.get();
            commands.entity(toolbelt_id).despawn_descendants();
            toolbelt_events.send(ToolbeltPopulateEvent {
                id: toolbelt_id,
                loadout: ToolbeltLoadout::Default,
            });
            if let Ok(mut position) = character_query.get_mut(character_id) {
                let center = tool.entry.bounds.center();
                position.0 = center.as_vec2().neg_y();
                if let Ok(elem) = find_element_at(center) {
                    if let Err(e) = elem.click() {
                        warn!("Failed to click taskbar entry: {:?}", e);
                    }
                }
            }
        }
    }
}
