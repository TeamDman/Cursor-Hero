use bevy::prelude::*;
use cursor_hero_fullscreen_tool_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::tool_spawning::{NoInputs, StartingState, ToolSpawnConfig};

pub struct FullscreenToolPopulatePlugin;

impl Plugin for FullscreenToolPopulatePlugin {
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
            PopulateToolbeltEvent::Default { toolbelt_id } => {
                ToolSpawnConfig::<_, NoInputs>::new(FullscreenTool, *toolbelt_id, event)
                    .guess_name(file!())
                    .guess_image(file!(), &asset_server, "webp")
                    .with_description("Toggles fullscreen mode.")
                    .with_starting_state(StartingState::Inactive)
                    .spawn(&mut commands);
            }
            _ => {}
        }
    }
}
