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
use cursor_hero_winutils::win_screen_capture::get_all_monitors;
use cursor_hero_winutils::win_screen_capture::get_monitor_infos;
use cursor_hero_math::prelude::Corner;
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
        let Ok(monitors) = get_all_monitors() else {
            warn!("No screens found");
            continue;
        };

        for monitor in monitors {
            for corner in Corner::variants() {
                let name = format!("{:?} Monitor {}", corner, monitor.info.id);
                ToolSpawnConfig::<WindowPositionTool, NoInputs>::new(
                    WindowPositionTool {
                        window_position: HostWindowPosition::Corner {
                            corner,
                            monitor: monitor.info.id,
                        },
                    },
                    event.id,
                    event,
                )
                .with_name(name.clone())
                .with_image(
                    asset_server.load(format!("textures/tools/window_position/{}.webp", name)),
                )
                .with_description("Moves the game window")
                .with_size(Vec2::new(100.0, 100.0))
                .with_starting_state(StartingState::Inactive)
                .spawn(&mut commands);
            }
            let name = format!("fullscreen_monitor_{}", monitor.info.id);
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
            .with_name(name.clone())
            .with_image(asset_server.load(format!("textures/tools/window_position/{}.webp", name)))
            .with_description("Moves the game window")
            .with_size(Vec2::new(100.0, 100.0))
            .with_starting_state(StartingState::Inactive)
            .spawn(&mut commands);
        }
    }
}

fn do_position(
    mut commands: Commands,
    tool_query: Query<(Entity, &WindowPositionTool), With<ActiveTool>>,
    mut window_query: Query<(&RawHandleWrapper, &mut Window), With<PrimaryWindow>>,
) {
    
    let Ok(monitor_infos) = get_monitor_infos() else {
        return;
    };

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
            HostWindowPosition::Corner {
                ref corner,
                monitor,
            } => {
                let Some(monitor) = monitor_infos.iter().find(|info| info.id == monitor) else {
                    warn!("No monitor found with id: {}", monitor);
                    continue;
                };
                debug!("Activating corner: {:?} on monitor: {}", corner, monitor.name);
                let dest_bounds = monitor.rect;
                    // IRect::from_center_size(IVec2::new(600, 600), IVec2::new(300, 400));
                window.position = WindowPosition::At(dest_bounds.top_right());
                commands.entity(tool_id).remove::<ActiveTool>();
            }
            _ => {}
        }
    }
}
