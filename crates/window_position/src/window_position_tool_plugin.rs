use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use cursor_hero_bevy::prelude::TopRightI;
use cursor_hero_toolbelt_types::prelude::PopulateToolbeltEvent;
use cursor_hero_toolbelt_types::toolbelt_types::ActiveTool;
use cursor_hero_toolbelt_types::toolbelt_types::ToolbeltLoadout;
use cursor_hero_tools::prelude::NoInputs;
use cursor_hero_tools::prelude::ToolSpawnConfig;
use cursor_hero_tools::tool_spawning::StartingState;
use cursor_hero_window_position_types::prelude::HostWindowPosition;
use cursor_hero_window_position_types::prelude::WindowPositionTool;
use cursor_hero_window_position_types::window_position_types::Corner;
pub struct WindowPositionToolPlugin;

impl Plugin for WindowPositionToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_toolbelts);
        app.add_systems(Update, do_position);
    }
}

fn populate_toolbelts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if event.loadout != ToolbeltLoadout::WindowPosition {
            continue;
        }

        ToolSpawnConfig::<WindowPositionTool, NoInputs>::new(
            WindowPositionTool {
                window_position: HostWindowPosition::Corner {
                    corner: Corner::TopRight,
                    monitor: 3,
                },
            },
            event.id,
            event,
        )
        .with_name("Top Right Monitor 3".to_string())
        .with_image(asset_server.load("textures/tools/window_position/fullscreen_monitor_1.webp"))
        .with_description("Swaps to taskbar tools")
        .with_size(Vec2::new(100.0, 100.0))
        .with_starting_state(StartingState::Inactive)
        .spawn(&mut commands);
    }
}

fn do_position(
    mut commands: Commands,
    tool_query: Query<(Entity, &WindowPositionTool), With<ActiveTool>>,
    mut window_query: Query<(&RawHandleWrapper, &mut Window), With<PrimaryWindow>>,
) {
    for tool in tool_query.iter() {
        let (tool_id, tool) = tool;
        let Ok(window) = window_query.get_single_mut() else {
            error!("No primary window found");
            return;
        };
        let (window_handle, mut window) = window;
        let win32handle = match window_handle.window_handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => handle,
            _ => panic!("Unsupported window handle"),
        };

        match tool.window_position {
            HostWindowPosition::Corner { ref corner, monitor } => {
                debug!("Activating corner: {:?} on monitor: {}", corner, monitor);
                let dest_bounds = IRect::from_center_size(IVec2::new(600,600), IVec2::new(300,400));
                window.position = WindowPosition::At(dest_bounds.top_right());
                commands.entity(tool_id).remove::<ActiveTool>();
            },
            _ => {}
        }
    }
}
