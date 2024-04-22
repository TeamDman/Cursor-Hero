use bevy::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

#[derive(Resource, Reflect, Clone)]
pub struct MemoryConfig {
    pub save_dir: String,
}

#[derive(Reflect, Default)]
pub struct MemoryPluginBuildConfig {
    pub main_character_memory_enabled: bool,
    pub primary_window_memory_enabled: bool,
    pub main_camera_memory_enabled: bool,
    pub voice_to_text_memory_enabled: bool,
    pub agent_observation_memory_enabled: bool,
    pub ui_data_memory_enabled: bool,
}

impl MemoryPluginBuildConfig {
    pub fn all_enabled() -> Self {
        Self {
            main_character_memory_enabled: true,
            primary_window_memory_enabled: true,
            main_camera_memory_enabled: true,
            voice_to_text_memory_enabled: true,
            agent_observation_memory_enabled: true,
            ui_data_memory_enabled: true,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum PersistError {
    Io(std::io::Error),
    WindowBounds(cursor_hero_winutils::win_window::WindowBoundsError),
    Query,
    Json(serde_json::Error),
}

#[derive(Debug)]
pub enum PersistSuccess {
    WritePerformed,
    Debounce,
    Cooldown,
    NoAction,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum RestoreError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Query,
}

#[derive(Debug)]
pub enum RestoreSuccess {
    Performed,
    NoAction,
}

#[derive(Eq, PartialEq)]
pub enum Usage {
    Persist,
    Restore,
}

pub fn get_persist_file(
    config: &MemoryConfig,
    file_name: &str,
    usage: Usage,
) -> Result<std::fs::File, std::io::Error> {
    let mut file_path = PathBuf::from(config.save_dir.clone());
    if usage == Usage::Persist && !file_path.exists() {
        std::fs::create_dir_all(&file_path)?;
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

pub fn write_to_disk<T>(mut file: File, data: &T) -> Result<PersistSuccess, PersistError>
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_string_pretty(data).map_err(PersistError::Json)?;
    file.write_all(serialized.as_bytes())
        .map_err(PersistError::Io)?;
    Ok(PersistSuccess::WritePerformed)
}

pub fn read_from_disk<T>(mut file: File) -> Result<T, RestoreError>
where
    T: serde::de::DeserializeOwned,
{
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(RestoreError::Io)?;
    let data = serde_json::from_str(&contents).map_err(RestoreError::Json)?;
    Ok(data)
}
