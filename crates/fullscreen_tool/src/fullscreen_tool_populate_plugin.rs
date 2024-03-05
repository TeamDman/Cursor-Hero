use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use cursor_hero_fullscreen_tool_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::tool_spawning::NoInputs;
use cursor_hero_tools::tool_spawning::ToolSpawnConfig;

pub struct FullscreenToolPopulatePlugin;

impl Plugin for FullscreenToolPopulatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_toolbelt_events);
    }
}

fn handle_toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for event in reader.read() {
        let ToolbeltLoadout::WindowPosition = event.loadout else {
            continue;
        };

        let mode = window_query
            .iter()
            .map(|w| w.mode)
            .next()
            .unwrap_or_default();
        let state = FullscreenTool::state_for_mode(mode);
        debug!("Window: {:?}, tool: {:?}", mode, state);
        ToolSpawnConfig::<_, NoInputs>::new(FullscreenTool, event.id, event)
            .with_src_path(file!().into())
            .guess_name(file!())
            .guess_image(file!(), &asset_server, "webp")
            .with_description("Toggles fullscreen mode.")
            .with_starting_state(state)
            .spawn(&mut commands);
    }
}
