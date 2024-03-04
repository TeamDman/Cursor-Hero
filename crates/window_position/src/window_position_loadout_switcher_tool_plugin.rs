use bevy::prelude::*;
use cursor_hero_toolbelt_types::toolbelt_types::{ActiveTool, ToolbeltLoadout};
use cursor_hero_tools::prelude::NoInputs;
use cursor_hero_tools::tool_spawning::StartingState;
use cursor_hero_tools::prelude::ToolSpawnConfig;
use cursor_hero_toolbelt_types::prelude::PopulateToolbeltEvent;
use cursor_hero_window_position_types::window_position_types::WindowPositionLoadoutSwitcherTool;

pub struct WindowPositionLoadoutSwitcherToolPlugin;

impl Plugin for WindowPositionLoadoutSwitcherToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_toolbelts);
        app.add_systems(Update, do_switch);
    }
}

fn populate_toolbelts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if event.loadout != ToolbeltLoadout::Default {
            continue;
        }
        ToolSpawnConfig::<WindowPositionLoadoutSwitcherTool, NoInputs>::new(
            WindowPositionLoadoutSwitcherTool,
            event.id,
            event,
        )
        .guess_name(file!())
        .with_image(asset_server.load("textures/tools/window_position.webp"))
        .with_description("Swaps to taskbar tools")
        .with_starting_state(StartingState::Inactive)
        .spawn(&mut commands);
    }
}

fn do_switch(
    mut commands: Commands,
    tool_query: Query<&Parent, (Added<ActiveTool>, With<WindowPositionLoadoutSwitcherTool>)>,
    mut toolbelt_events: EventWriter<PopulateToolbeltEvent>,
) {
    for toolbelt_id in tool_query.iter() {
        let toolbelt_id = toolbelt_id.get();
        commands.entity(toolbelt_id).despawn_descendants();
        toolbelt_events.send(PopulateToolbeltEvent {
            id: toolbelt_id,
            loadout: ToolbeltLoadout::WindowPosition,
        });
    }
}
