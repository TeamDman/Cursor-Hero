use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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
pub enum Usage {
    Persist,
    Restore,
}

pub fn get_persist_file(
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

pub fn write_to_disk<T>(mut file: File, data: T) -> Result<PersistSuccess, PersistError>
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_string_pretty(&data).map_err(PersistError::Json)?;
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
