use bevy::prelude::*;
use cursor_hero_memory_types::prelude::*;
use serde::Deserialize;
use serde::Serialize;

pub struct {{crate_name_pascal}}MemoryPlugin;

impl Plugin for {{crate_name_pascal}}MemoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource({{crate_name_pascal}}MemoryConfig::default());
        app.add_systems(Update, persist.pipe(handle_persist_errors));
        app.add_systems(
            Startup,
            (apply_deferred, restore.pipe(handle_restore_errors)).chain(),
        );
    }
}
const PERSIST_FILE_NAME: &str = "{{crate_name}}.json";

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct DiskData {
    // FILL ME UP
}

fn persist(
    mut config: ResMut<{{crate_name_pascal}}MemoryConfig>,
    memory_config: Res<MemoryConfig>,
    mut debounce: Local<Option<DiskData>>,
    time: Res<Time>,
    // QUERY YOUR DATA
) -> Result<PersistSuccess, PersistError> {
    if !config.debounce_timer.tick(time.delta()).just_finished() {
        return Ok(PersistSuccess::Cooldown);
    }

    // BUILD YOUR DISK DATA
    let data = todo!();

    if debounce.is_none() || debounce.as_ref().unwrap() != &data {
        let file = get_persist_file(memory_config.as_ref(), PERSIST_FILE_NAME, Usage::Persist)
            .map_err(PersistError::Io)?;
        write_to_disk(file, &data)?;
        *debounce = Some(data);
        Ok(PersistSuccess::WritePerformed)
    } else {
        Ok(PersistSuccess::Debounce)
    }
}

fn restore(
    memory_config: Res<MemoryConfig>,
    // MUT QUERY YOUR DATA
) -> Result<RestoreSuccess, RestoreError> {
    let file = get_persist_file(memory_config.as_ref(), PERSIST_FILE_NAME, Usage::Restore)
        .map_err(RestoreError::Io)?;
    let mut data: DiskData = read_from_disk(file)?;

    info!("Restoring {{crate_name_pascal}}");

    // RESTORE YOUR DATA
    todo!();

    Ok(RestoreSuccess::Performed)
}
