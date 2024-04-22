use bevy::prelude::*;
use cursor_hero_character_types::prelude::*;

use cursor_hero_memory_types::prelude::*;
use cursor_hero_toolbelt_types::toolbelt_types::Toolbelt;
use cursor_hero_toolbelt_types::toolbelt_types::ToolbeltPopulateEvent;
use serde::Deserialize;
use serde::Serialize;

pub struct MainCharacterMemoryPlugin;

impl Plugin for MainCharacterMemoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MainCharacterMemoryConfig::default());
        app.add_systems(Update, persist.pipe(handle_persist_errors));
        app.add_systems(
            Update,
            (apply_deferred, restore.pipe(handle_restore_errors)).chain(),
        );
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
    character_position: Vec3,
    toolbelt: Toolbelt,
}

fn persist(
    mut config: ResMut<MainCharacterMemoryConfig>,
    memory_config: Res<MemoryConfig>,
    mut debounce: Local<Option<DiskData>>,
    time: Res<Time>,
    character_query: Query<(&Transform, &Children), With<MainCharacter>>,
    toolbelt_query: Query<&Toolbelt>,
) -> Result<PersistSuccess, PersistError> {
    if !config.debounce_timer.tick(time.delta()).just_finished() {
        return Ok(PersistSuccess::Cooldown);
    }

    let character = character_query
        .get_single()
        .map_err(|_| PersistError::Query)?;
    let (character_transform, character_children) = character;
    let character_position = character_transform.translation;

    let mut found = None;
    for child in character_children.iter() {
        match (found, toolbelt_query.get(*child)) {
            (None, Ok(toolbelt)) => {
                found = Some(toolbelt);
            }
            (Some(_), Ok(_)) => {
                return Err(PersistError::Query);
            }
            _ => {}
        }
    }
    let toolbelt = *found.ok_or(PersistError::Query)?;

    let data = DiskData {
        character_position,
        toolbelt,
    };
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
    mut character_query: Query<(&mut Transform, &Children), Added<MainCharacter>>,
    mut toolbelt_query: Query<&mut Toolbelt>,
    mut commands: Commands,
    mut toolbelt_events: EventWriter<ToolbeltPopulateEvent>,
) -> Result<RestoreSuccess, RestoreError> {
    let Ok(character) = character_query.get_single_mut() else {
        return Ok(RestoreSuccess::NoAction);
    };
    let (mut character_transform, character_children) = character;
    let mut toolbelt_id = None;
    for child in character_children.iter() {
        match (toolbelt_id, toolbelt_query.contains(*child)) {
            (None, true) => {
                toolbelt_id = Some(child);
            }
            (Some(_), true) => {
                return Err(RestoreError::Query);
            }
            _ => {}
        }
    }
    let toolbelt_id = *toolbelt_id.ok_or(RestoreError::Query)?;
    let mut toolbelt = toolbelt_query
        .get_mut(toolbelt_id)
        .map_err(|_| RestoreError::Query)?;

    let file = get_persist_file(memory_config.as_ref(), PERSIST_FILE_NAME, Usage::Restore)
        .map_err(RestoreError::Io)?;
    let data: DiskData = read_from_disk(file)?;

    info!(
        "Restoring main character position to {:?}",
        data.character_position
    );
    character_transform.translation = data.character_position;

    info!("Restoring toolbelt to {:?}", data.toolbelt);
    *toolbelt = data.toolbelt;
    commands.entity(toolbelt_id).despawn_descendants();
    toolbelt_events.send(ToolbeltPopulateEvent {
        id: toolbelt_id,
        loadout: data.toolbelt.loadout,
    });
    // layout is going to get clobbered to defaults by toolbelt_properties_plugin
    // this is fine for now since there are no scenarios where a loadout isn't using its default layout

    Ok(RestoreSuccess::Performed)
}
