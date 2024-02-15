#![feature(stmt_expr_attributes)]

mod main_camera_memory_plugin;
mod main_character_memory_plugin;
mod primary_window_memory_plugin;
pub mod voice_to_text_memory_plugin;
pub mod agent_observation_memory_plugin;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use bevy::prelude::*;

pub struct MemoryPlugin;

impl Plugin for MemoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            main_character_memory_plugin::MainCharacterMemoryPlugin,
            primary_window_memory_plugin::PrimaryWindowMemoryPlugin,
            main_camera_memory_plugin::MainCameraMemoryPlugin,
            voice_to_text_memory_plugin::VoiceToTextMemoryPlugin,
            agent_observation_memory_plugin::AgentObservationMemoryPlugin,
        ));
    }
}

#[derive(Debug)]
enum PersistError {
    Io(std::io::Error),
    WindowBounds(cursor_hero_winutils::win_window::WindowBoundsError),
    Query,
    Json(serde_json::Error),
}

#[derive(Debug)]
enum PersistSuccess {
    WritePerformed,
    Debounce,
    Cooldown,
    NoAction,
}

#[derive(Debug)]
enum RestoreError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

#[derive(Debug)]
enum RestoreSuccess {
    Performed,
    NoAction,
}
enum Usage {
    Persist,
    Restore,
}

fn get_persist_file(
    current_path: &str,
    file_name: &str,
    usage: Usage,
) -> Result<std::fs::File, std::io::Error> {
    let mut file_path = PathBuf::new();

    #[cfg(debug_assertions)]
    {
        let dir = Path::new(current_path).parent().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Parent not found",
        ))?;
        file_path.push(dir);
    }

    file_path.push(file_name);

    let file = match usage {
        Usage::Persist => OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)?,
        Usage::Restore => OpenOptions::new().read(true).open(file_path)?,
    };
    Ok(file)
}

fn write_to_disk<T>(mut file: File, data: T) -> Result<PersistSuccess, PersistError>
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_string_pretty(&data).map_err(PersistError::Json)?;
    file.write_all(serialized.as_bytes())
        .map_err(PersistError::Io)?;
    Ok(PersistSuccess::WritePerformed)
}

fn read_from_disk<T>(mut file: File) -> Result<T, RestoreError>
where
    T: serde::de::DeserializeOwned,
{
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(RestoreError::Io)?;
    let data = serde_json::from_str(&contents).map_err(RestoreError::Json)?;
    Ok(data)
}
