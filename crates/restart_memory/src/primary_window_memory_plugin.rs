use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use bevy::winit::WinitWindows;
use cursor_hero_winutils::win_window::get_window_inner_bounds;

pub struct PrimaryWindowMemoryPlugin;

impl Plugin for PrimaryWindowMemoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrimaryWindowMemoryConfig::default())
            .add_systems(Update, note_window_info.pipe(handle_persist_errors));
    }
}

#[derive(Debug, Resource)]
pub struct PrimaryWindowMemoryConfig {
    pub save_bounds: bool,
    pub timer: Timer,
}
impl Default for PrimaryWindowMemoryConfig {
    fn default() -> Self {
        Self {
            save_bounds: true,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

#[derive(Debug)]
enum PersistError {
    Io(std::io::Error),
    WindowBounds(cursor_hero_winutils::win_window::WindowBoundsError),
    StringFormatting,
    Query,
}

#[derive(Debug)]
enum PersistSuccess {
    WritePerformed,
    Debounce,
    Cooldown,
    Disabled,
}

fn note_window_info(
    mut config: ResMut<PrimaryWindowMemoryConfig>,
    time: Res<Time>,
    window_query: Query<(Entity, &RawHandleWrapper, &Window), With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
    mut debounce: Local<(Vec2, IVec2)>,
) -> Result<PersistSuccess, PersistError> {
    if !config.timer.tick(time.delta()).just_finished() {
        return Ok(PersistSuccess::Cooldown);
    }
    if !config.save_bounds {
        return Ok(PersistSuccess::Disabled);
    }

    let (window_id, window_handle, window) =
        window_query.get_single().map_err(|_| PersistError::Query)?;

    let winit_window = winit_windows
        .get_window(window_id)
        .ok_or(PersistError::Query)?;

    if winit_window.is_minimized().unwrap_or(false) {
        return Ok(PersistSuccess::Disabled);
    }
    let resolution = Vec2::new(
        window.resolution.physical_width() as f32,
        window.resolution.physical_height() as f32,
    );
    let position = match window.position {
        WindowPosition::At(position) => position,
        _ => {
            let hwnd = match window_handle.window_handle {
                raw_window_handle::RawWindowHandle::Win32(handle) => handle.hwnd as isize,
                _ => return Ok(PersistSuccess::Disabled),
            };
            get_window_inner_bounds(hwnd)
                .map_err(PersistError::WindowBounds)?
                .size()
                .as_ivec2()
        }
    };

    if (*debounce).0 != resolution || debounce.1 != position {
        // only save if not minimized
        if position.x != -32000 && position.y != -32000 {
            persist_window_bounds(resolution, position)?;
        }
        *debounce = (resolution, position);
        return Ok(PersistSuccess::WritePerformed);
    }
    Ok(PersistSuccess::Debounce)
}

fn handle_persist_errors(In(result): In<Result<PersistSuccess, PersistError>>) {
    if let Err(e) = result {
        error!("persist error occurred: {:?}", e);
    } else if let Ok(PersistSuccess::WritePerformed) = result {
        info!("persisted window bounds");
    }
}

// The function that persists the window bounds and position to a file.
fn persist_window_bounds(resolution: Vec2, position: IVec2) -> Result<(), PersistError> {
    let mut main_rs = std::fs::read_to_string("src/main.rs").map_err(PersistError::Io)?;
    let begin_resolution = "%BEGIN_RESOLUTION%";
    let end_resolution = "%END_RESOLUTION%";
    let begin_position = "%BEGIN_POSITION%";
    let end_position = "%END_POSITION%";
    let indent = "                        ";
    let resolution_replace = format!(
        "{}\n{}{}.0,\n{}{}.0\n{}// {}",
        begin_resolution, indent, resolution.x, indent, resolution.y, indent, end_resolution
    );
    let position_replace = format!(
        "{}\n{}{},\n{}{}\n{}// {}",
        begin_position, indent, position.x, indent, position.y, indent, end_position
    );

    let begin_resolution_index = main_rs
        .find(begin_resolution)
        .ok_or(PersistError::StringFormatting)?;
    let end_resolution_index = main_rs
        .find(end_resolution)
        .ok_or(PersistError::StringFormatting)?;
    main_rs.replace_range(
        begin_resolution_index..end_resolution_index + end_resolution.len(),
        &resolution_replace,
    );

    let begin_position_index = main_rs
        .find(begin_position)
        .ok_or(PersistError::StringFormatting)?;
    let end_position_index = main_rs
        .find(end_position)
        .ok_or(PersistError::StringFormatting)?;
    main_rs.replace_range(
        begin_position_index..end_position_index + end_position.len(),
        &position_replace,
    );

    std::fs::write("src/main.rs", main_rs).map_err(PersistError::Io)
}
