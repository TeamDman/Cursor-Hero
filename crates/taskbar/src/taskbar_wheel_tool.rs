use bevy::prelude::*;
use bevy_xpbd_2d::components::Position;
use cursor_hero_bevy::NegativeYVec2;
use cursor_hero_screen::get_image::get_image;
use cursor_hero_screen::get_image::ScreensToImageParam;
use cursor_hero_toolbelt::types::*;
use cursor_hero_tools::prelude::*;
use cursor_hero_winutils::ui_automation::get_taskbar;
use cursor_hero_winutils::ui_automation::TaskbarEntry;

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

#[derive(Component, Reflect, Default)]
struct TaskbarWheelTool;

#[derive(Component, Reflect)]
struct TaskbarEntryTool {
    entry: TaskbarEntry,
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
    access: ScreensToImageParam,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Default { toolbelt_id } = event {
            ToolSpawnConfig::<TaskbarWheelTool, NoInputs>::new(
                TaskbarWheelTool,
                *toolbelt_id,
                event,
            )
            .guess_name(file!())
            .guess_image(file!(), &asset_server)
            .with_description("Swaps to taskbar tools")
            .with_starting_state(StartingState::Inactive)
            .spawn(&mut commands);
        }
        if let PopulateToolbeltEvent::Taskbar { toolbelt_id } = event {
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
                    *toolbelt_id,
                    event,
                )
                .with_name(entry.name)
                .with_description("Swaps to taskbar tools")
                .with_image(asset_server.add(image))
                .with_size(entry.bounds.size().as_vec2())
                .with_starting_state(StartingState::Inactive)
                .spawn(&mut commands);
            }
        }
    }
}

fn tick_wheel_switcher(
    mut commands: Commands,
    tool_query: Query<&Parent, (Added<ActiveTool>, With<TaskbarWheelTool>)>,
    mut toolbelt_events: EventWriter<PopulateToolbeltEvent>,
) {
    for toolbelt_id in tool_query.iter() {
        let toolbelt_id = toolbelt_id.get();
        commands.entity(toolbelt_id).despawn_descendants();
        toolbelt_events.send(PopulateToolbeltEvent::Taskbar { toolbelt_id });
    }
}

fn tick_taskbar_switcher(
    mut commands: Commands,
    tool_query: Query<(&Parent, &TaskbarEntryTool), Added<ActiveTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut character_query: Query<&mut Position>,
    mut toolbelt_events: EventWriter<PopulateToolbeltEvent>,
) {
    for (toolbelt_id, tool) in tool_query.iter() {
        let toolbelt_id = toolbelt_id.get();
        if let Ok(character_id) = toolbelt_query.get(toolbelt_id) {
            info!("Switching toolbelt {:?} to default tools", toolbelt_id);
            let character_id = character_id.get();
            commands.entity(toolbelt_id).despawn_descendants();
            toolbelt_events.send(PopulateToolbeltEvent::Default { toolbelt_id });
            if let Ok(mut position) = character_query.get_mut(character_id) {
                position.0 = tool.entry.bounds.center().as_vec2().neg_y();
            }
        }
    }
}
