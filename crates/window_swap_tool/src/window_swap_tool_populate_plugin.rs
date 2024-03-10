use bevy::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::tool_spawning::ToolSpawnConfig;
use cursor_hero_window_swap_tool_types::prelude::*;

pub struct WindowSwapToolPopulatePlugin;

impl Plugin for WindowSwapToolPopulatePlugin {
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
        let ToolbeltPopulateEvent {
            loadout: ToolbeltLoadout::WindowPosition,
            ..
        } = event
        else {
            continue;
        };
        ToolSpawnConfig::<_, WindowSwapToolAction>::new(WindowSwapTool, event.id, event)
            .with_src_path(file!().into())
            .guess_name(file!())
            .guess_image(file!(), &asset_server, "webp")
            .with_description("Swap the positions of windows.")
            .spawn(&mut commands);
    }
}
