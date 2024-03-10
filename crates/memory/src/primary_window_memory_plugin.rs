use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use bevy::window::WindowMode;
use bevy::window::WindowResolution;
use bevy::winit::WinitWindows;
use cursor_hero_memory_types::prelude::*;
use cursor_hero_winutils::win_window::get_window_inner_bounds;
use serde::Deserialize;
use serde::Serialize;

pub struct PrimaryWindowMemoryPlugin;

// TODO: remember maximized status

impl Plugin for PrimaryWindowMemoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrimaryWindowMemoryConfig::default());
        app.add_systems(Update, persist.pipe(handle_persist_errors));
        app.add_systems(Update, restore.pipe(handle_restore_errors));
    }
}

const PERSIST_FILE_NAME: &str = "primary_window.json";

// not moved to lib to ensure log contains this module name
fn handle_persist_errors(In(result): In<Result<PersistSuccess, PersistError>>) {
    if let Err(e) = result {
        error!("Persist error occurred: {:?}", e);
    } else if let Ok(PersistSuccess::WritePerformed) = result {
        debug!("Persisted succeeded");
    }
}

fn handle_restore_errors(In(result): In<Result<RestoreSuccess, RestoreError>>) {
    if let Err(e) = result {
        error!("Restore error occurred: {:?}", e);
    } else if let Ok(RestoreSuccess::Performed) = result {
        info!("Restore succeeded");
    }
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
struct PrimaryWindowMemoryConfig {
    pub debounce_timer: Timer,
}
impl Default for PrimaryWindowMemoryConfig {
    fn default() -> Self {
        Self {
            debounce_timer: Timer::from_seconds(10.0, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
struct DiskData {
    resolution: Vec2,
    position: IVec2,
    mode: WindowMode,
}

fn persist(
    mut config: ResMut<PrimaryWindowMemoryConfig>,
    time: Res<Time>,
    window_query: Query<(Entity, &RawHandleWrapper, &Window), With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
    mut debounce: Local<Option<DiskData>>,
) -> Result<PersistSuccess, PersistError> {
    if !config.debounce_timer.tick(time.delta()).just_finished() {
        return Ok(PersistSuccess::Cooldown);
    }

    let (window_id, window_handle, window) =
        window_query.get_single().map_err(|_| PersistError::Query)?;

    let winit_window = winit_windows
        .get_window(window_id)
        .ok_or(PersistError::Query)?;

    if winit_window.is_minimized().unwrap_or(false) {
        return Ok(PersistSuccess::NoAction);
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
                _ => return Ok(PersistSuccess::NoAction),
            };
            get_window_inner_bounds(hwnd)
                .map_err(PersistError::WindowBounds)?
                .size()
        }
    };

    let data = DiskData {
        resolution,
        position,
        mode: window.mode,
    };
    if debounce.is_none() || debounce.as_ref().unwrap() != &data {
        let minimized = position.x == -32000 || position.y == -32000;
        if minimized {
            return Ok(PersistSuccess::NoAction);
        }
        let file = get_persist_file(file!(), PERSIST_FILE_NAME, Usage::Persist)
            .map_err(PersistError::Io)?;
        write_to_disk(file, data)?;
        *debounce = Some(data);
        Ok(PersistSuccess::WritePerformed)
    } else {
        Ok(PersistSuccess::Debounce)
    }
}

fn restore(
    mut window_query: Query<&mut Window, Added<PrimaryWindow>>,
) -> Result<RestoreSuccess, RestoreError> {
    let Ok(mut window) = window_query.get_single_mut() else {
        return Ok(RestoreSuccess::NoAction);
    };
    let file =
        get_persist_file(file!(), PERSIST_FILE_NAME, Usage::Restore).map_err(RestoreError::Io)?;
    let data = read_from_disk::<DiskData>(file)?;
    window.resolution = WindowResolution::from(data.resolution);
    window.position = WindowPosition::At(data.position);
    window.mode = data.mode;

    Ok(RestoreSuccess::Performed)
}
