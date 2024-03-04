use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use bevy::window::WindowMode;
use bevy::window::WindowResolution;
use cursor_hero_bevy::prelude::BottomRightI;
use cursor_hero_bevy::prelude::CornerOfIRect;
use cursor_hero_bevy::prelude::IExpandable;
use cursor_hero_bevy::prelude::IRectScale;
use cursor_hero_bevy::prelude::TopLeftI;
use cursor_hero_bevy::prelude::TopRightI;
use cursor_hero_bevy::prelude::TranslateIVec2;
use cursor_hero_math::prelude::Corner;
use cursor_hero_toolbelt_types::prelude::PopulateToolbeltEvent;
use cursor_hero_toolbelt_types::toolbelt_types::ActiveTool;
use cursor_hero_toolbelt_types::toolbelt_types::ToolbeltLoadout;
use cursor_hero_tools::prelude::NoInputs;
use cursor_hero_tools::prelude::ToolSpawnConfig;
use cursor_hero_tools::tool_spawning::StartingState;
use cursor_hero_window_position_types::prelude::HostWindowPosition;
use cursor_hero_window_position_types::prelude::WindowPositionTool;
use cursor_hero_winutils::win_mouse::set_cursor_position;
use cursor_hero_winutils::win_screen_capture::get_all_monitors;
use cursor_hero_winutils::win_screen_capture::get_monitor_infos;
use cursor_hero_winutils::win_screen_capture::Monitor;
use image::ImageBuffer;
use image::Rgba;
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
    mut textures: ResMut<Assets<Image>>,
) {
    for event in reader.read() {
        if event.loadout != ToolbeltLoadout::WindowPosition {
            continue;
        }
        let Ok(monitors) = get_all_monitors() else {
            warn!("No monitors found");
            continue;
        };
        let icon_size = UVec2::new(500, 500);

        let mut world = IRect::from_corners(
            monitors
                .iter()
                .map(|monitor| monitor.info.rect.top_left())
                .reduce(|a, b| a.min(b))
                .unwrap_or_default(),
            monitors
                .iter()
                .map(|monitor| monitor.info.rect.bottom_right())
                .reduce(|a, b| a.max(b))
                .unwrap_or_default(),
        );

        // expand it to be square aspect ratio
        if world.size().x > world.size().y {
            world = world.scale(Vec2::new(
                1.0,
                world.size().x as f32 / world.size().y as f32,
            ));
        } else {
            world = world.scale(Vec2::new(
                world.size().y as f32 / world.size().x as f32,
                1.0,
            ));
        }

        for monitor in monitors.iter() {
            for corner in Corner::variants() {
                let name = format!("{:?} Monitor {}", corner, monitor.info.name);

                let image = image_for_monitor_corner(icon_size, world, monitor, &corner);
                let texture = textures.add(image);

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
                .with_image(texture)
                .with_description("Moves the game window")
                .with_size(Vec2::new(100.0, 100.0))
                .with_starting_state(StartingState::Inactive)
                .spawn(&mut commands);
            }
            let name = format!("fullscreen_monitor_{}", monitor.info.id);
            ToolSpawnConfig::<WindowPositionTool, NoInputs>::new(
                WindowPositionTool {
                    window_position: HostWindowPosition::Fullscreen {
                        monitor: monitor.info.id,
                    },
                },
                event.id,
                event,
            )
            .with_name(name.clone())
            .with_image(textures.add(image_for_monitor(icon_size, world, monitor)))
            .with_description("Moves the game window")
            .with_size(Vec2::new(100.0, 100.0))
            .with_starting_state(StartingState::Inactive)
            .spawn(&mut commands);
        }
    }
}

fn image_for_monitor_corner(
    icon_size: UVec2,
    world: IRect,
    monitor: &Monitor,
    corner: &Corner,
) -> Image {
    let mut imgbuf =
        ImageBuffer::from_pixel(icon_size.x, icon_size.y, Rgba([173u8, 216u8, 230u8, 255u8])); // Light blue background

    let scale = icon_size.as_vec2() / world.size().as_vec2();
    let monitor_icon_region = monitor.info.rect.translate(&-world.min).scale(scale);
    debug!(
        "Monitor icon region: {:?}, scale: {:?}",
        monitor_icon_region, scale
    );

    let dest_icon_region = IRect::from_corners(
        monitor_icon_region.center(),
        corner.of(&monitor_icon_region),
    );
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if monitor_icon_region.contains(IVec2::new(x as i32, y as i32)) {
            *pixel = Rgba([255u8, 0u8, 0u8, 255u8]);
        }
        if dest_icon_region.contains(IVec2::new(x as i32, y as i32)) {
            *pixel = Rgba([0u8, 255u8, 0u8, 255u8]);
        }
    }
    let image = Image::from_dynamic(imgbuf.into(), true);
    image
}

fn image_for_monitor(icon_size: UVec2, world: IRect, monitor: &Monitor) -> Image {
    let mut imgbuf =
        ImageBuffer::from_pixel(icon_size.x, icon_size.y, Rgba([173u8, 216u8, 230u8, 255u8])); // Light blue background

    let scale = icon_size.as_vec2() / world.size().as_vec2();
    let monitor_icon_region = monitor.info.rect.translate(&-world.min).scale(scale);
    debug!(
        "Monitor icon region: {:?}, scale: {:?}",
        monitor_icon_region, scale
    );
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if monitor_icon_region.contains(IVec2::new(x as i32, y as i32)) {
            *pixel = Rgba([0u8, 0u8, 255u8, 255u8]);
        }
    }
    let image = Image::from_dynamic(imgbuf.into(), true);
    image
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
                debug!(
                    "Activating corner: {:?} on monitor: {}",
                    corner, monitor.name
                );
                let dest_bounds = IRect::from_corners(
                    monitor.work_area.center(),
                    corner.of(&monitor.work_area)
                        - ((corner.of(&monitor.work_area) - monitor.work_area.center())
                            .as_vec2()
                            .normalize()
                            * 100.0)
                            .as_ivec2(),
                );
                window.mode = WindowMode::Windowed;
                window.position = WindowPosition::At(dest_bounds.top_left());
                window.resolution =
                    WindowResolution::new(dest_bounds.width() as f32, dest_bounds.height() as f32);
                    
                if let Err(e) = set_cursor_position(dest_bounds.center()) {
                    warn!("Failed to set cursor position: {}", e);
                }
                commands.entity(tool_id).remove::<ActiveTool>();
            }
            HostWindowPosition::Fullscreen { monitor } => {
                let Some(monitor) = monitor_infos.iter().find(|info| info.id == monitor) else {
                    warn!("No monitor found with id: {}", monitor);
                    continue;
                };
                debug!("Activating fullscreen on monitor: {}", monitor.name);
                window.position = WindowPosition::At(monitor.work_area.top_left());
                window.resolution = WindowResolution::new(
                    monitor.work_area.width() as f32,
                    monitor.work_area.height() as f32,
                );
                window.mode = WindowMode::BorderlessFullscreen;
                if let Err(e) = set_cursor_position(monitor.work_area.center()) {
                    warn!("Failed to set cursor position: {}", e);
                }
                commands.entity(tool_id).remove::<ActiveTool>();
            }
        }
    }
}
