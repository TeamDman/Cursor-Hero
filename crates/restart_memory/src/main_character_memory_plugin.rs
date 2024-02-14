use bevy::prelude::*;
use cursor_hero_character_types::prelude::*;

use serde::Deserialize;
use serde::Serialize;

use crate::get_persist_file;
use crate::read_from_disk;
use crate::write_to_disk;
use crate::PersistError;
use crate::PersistSuccess;
use crate::RestoreError;
use crate::RestoreSuccess;
use crate::Usage;

pub struct MainCharacterMemoryPlugin;

impl Plugin for MainCharacterMemoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MainCharacterMemoryConfig::default());
        app.add_systems(Update, persist.pipe(handle_persist_errors));
        app.add_systems(Update, restore.pipe(handle_restore_errors));
    }
}
const PERSIST_FILE_NAME: &str = "main_character.json";

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
struct MainCharacterMemoryConfig {
    pub debounce_timer: Timer,
}
impl Default for MainCharacterMemoryConfig {
    fn default() -> Self {
        Self {
            debounce_timer: Timer::from_seconds(10.0, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
struct DiskData {
    position: Vec3,
}

fn persist(
    mut config: ResMut<MainCharacterMemoryConfig>,
    mut debounce: Local<Option<DiskData>>,
    time: Res<Time>,
    character_query: Query<&Transform, With<MainCharacter>>,
) -> Result<PersistSuccess, PersistError> {
    if !config.debounce_timer.tick(time.delta()).just_finished() {
        return Ok(PersistSuccess::Cooldown);
    }

    let character_transform = character_query
        .get_single()
        .map_err(|_| PersistError::Query)?;
    let character_position = character_transform.translation;

    let data = DiskData {
        position: character_position,
    };
    if debounce.is_none() || debounce.as_ref().unwrap() != &data {
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
    mut character_query: Query<&mut Transform, Added<MainCharacter>>,
) -> Result<RestoreSuccess, RestoreError> {
    let Ok(mut character_transform) = character_query.get_single_mut() else {
        return Ok(RestoreSuccess::NoAction);
    };
    let file =
        get_persist_file(file!(), PERSIST_FILE_NAME, Usage::Restore).map_err(RestoreError::Io)?;
    let data: DiskData = read_from_disk(file)?;
    info!("Restoring main character position to {:?}", data.position);
    character_transform.translation = data.position;
    Ok(RestoreSuccess::Performed)
}
