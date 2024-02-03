use bevy::prelude::*;
use cursor_hero_movement_tool_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::tool_spawning::ToolSpawnConfig;

pub struct SpawnMovementToolPlugin;

impl Plugin for SpawnMovementToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_toolbelt_events);
    }
}

fn handle_toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        match event {
            PopulateToolbeltEvent::Default { toolbelt_id }
            | PopulateToolbeltEvent::Inspector { toolbelt_id }
            | PopulateToolbeltEvent::Taskbar { toolbelt_id }
            | PopulateToolbeltEvent::Keyboard { toolbelt_id } => {
                ToolSpawnConfig::<_, MovementToolAction>::new(
                    MovementTool::default(),
                    *toolbelt_id,
                    event,
                )
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Go faster, reach further")
                .spawn(&mut commands);
            }
            PopulateToolbeltEvent::Agent { toolbelt_id } => {
                ToolSpawnConfig::<_, MovementToolAction>::new(
                    MovementTool::default(),
                    *toolbelt_id,
                    event,
                )
                .with_input_map(None)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Go faster, reach further")
                .spawn(&mut commands);
            }
        }
    }
}
