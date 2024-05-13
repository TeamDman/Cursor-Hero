use bevy::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::tool_spawning::ToolSpawnConfig;
use cursor_hero_zoom_tool_types::prelude::*;

pub struct ZoomToolPopulatePlugin;

impl Plugin for ZoomToolPopulatePlugin {
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
            loadout: ToolbeltLoadout::Default,
            ..
        } = event
        else {
            continue;
        };
        ToolSpawnConfig::<_, ZoomToolAction>::new(ZoomTool::default(), event.id, event)
            .with_src_path(file!().into())
            .guess_name(file!())
            .guess_image(file!(), &asset_server, "png")
            .with_description("Send scroll events")
            .spawn(&mut commands);
    }
}
