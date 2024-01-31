use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_winutils::ui_automation::get_taskbar;

pub struct ObservationToolPlugin;

impl Plugin for ObservationToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ObservationTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(Update, tick);
    }
}

#[derive(Component, Reflect, Default)]
struct ObservationTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Inspector { toolbelt_id } = event {
            ToolSpawnConfig::<ObservationTool, NoInputs>::new(ObservationTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Logs information about the environment to the console.")
                .with_starting_state(StartingState::Inactive)
                .spawn(&mut commands);
        }
    }
}

fn tick(
    mut commands: Commands,
    tool_query: Query<Entity, (Added<ActiveTool>, With<ObservationTool>)>,
) {
    for tool_id in tool_query.iter() {
        commands.entity(tool_id).remove::<ActiveTool>();
        let Ok(taskbar) = get_taskbar() else {
            warn!("Failed to get taskbar");
            continue;
        };
        info!("Taskbar entries: {:?}", &taskbar.entries);
    }
}
