use bevy::prelude::*;
use cursor_hero_movement_tool_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::tool_spawning::ToolSpawnConfig;

pub struct MovementToolPopulatePlugin;

impl Plugin for MovementToolPopulatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_toolbelt_events);
    }
}

fn handle_toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        match event.loadout {
            ToolbeltLoadout::Default
            | ToolbeltLoadout::Inspector
            | ToolbeltLoadout::Taskbar
            | ToolbeltLoadout::WindowPosition
            | ToolbeltLoadout::Keyboard => {
                ToolSpawnConfig::<_, MovementToolAction>::new(
                    MovementTool::default(),
                    event.id,
                    event,
                )
                .with_src_path(file!().into())
                .guess_name(file!())
                .guess_image(file!(), &asset_server, "png")
                .with_description("Go faster, reach further")
                .spawn(&mut commands);
            }
            ToolbeltLoadout::Agent => {
                ToolSpawnConfig::<_, MovementToolAction>::new(
                    MovementTool::default(),
                    event.id,
                    event,
                )
                .with_src_path(file!().into())
                .with_input_map(None)
                .guess_name(file!())
                .guess_image(file!(), &asset_server, "png")
                .with_description("Go faster, reach further")
                .spawn(&mut commands);
            }
            _ => {}
        }
    }
}
