use bevy::prelude::*;
use bevy::utils::HashMap;
use cursor_hero_character_types::prelude::*;

use cursor_hero_memory_types::prelude::*;
use cursor_hero_observation_types::observation_types::ObservationBuffer;
use cursor_hero_observation_types::observation_types::SomethingObservableHappenedEvent;
use serde::Deserialize;
use serde::Serialize;

pub struct AgentObservationMemoryPlugin;

impl Plugin for AgentObservationMemoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MainCharacterMemoryConfig::default());
        app.add_systems(Update, persist.pipe(handle_persist_errors));
        app.add_systems(Update, restore.pipe(handle_restore_errors));
    }
}
const PERSIST_FILE_NAME: &str = "agent_memory.json";

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
    pub persist_cooldown: Timer,
}
impl Default for MainCharacterMemoryConfig {
    fn default() -> Self {
        Self {
            persist_cooldown: Timer::from_seconds(10.0, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
struct DiskData {
    observations_by_observer_name: HashMap<String, ObservationBuffer>,
}

fn persist(
    mut config: ResMut<MainCharacterMemoryConfig>,
    memory_config: Res<MemoryConfig>,
    mut debounce: Local<Option<DiskData>>,
    time: Res<Time>,
    agent_query: Query<(&Name, &ObservationBuffer), With<AgentCharacter>>,
) -> Result<PersistSuccess, PersistError> {
    if !config.persist_cooldown.tick(time.delta()).just_finished() {
        return Ok(PersistSuccess::Cooldown);
    }

    let mut data = DiskData::default();
    for agent in agent_query.iter() {
        data.observations_by_observer_name
            .insert(agent.0.as_str().to_string(), agent.1.clone());
    }

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
    mut agent_query: Query<(Entity, &Name, &mut ObservationBuffer), Added<AgentCharacter>>,
    mut observation_events: EventWriter<SomethingObservableHappenedEvent>,
) -> Result<RestoreSuccess, RestoreError> {
    if agent_query.is_empty() {
        return Ok(RestoreSuccess::NoAction);
    }

    let file = get_persist_file(memory_config.as_ref(), PERSIST_FILE_NAME, Usage::Restore)
        .map_err(RestoreError::Io)?;
    let mut data: DiskData = read_from_disk(file)?;
    info!(
        "Restoring agent memories, found {} entries",
        data.observations_by_observer_name.len()
    );
    for agent in agent_query.iter_mut() {
        let (agent_id, agent_name, mut agent_buffer) = agent;
        let agent_name = agent_name.as_str();
        // Each agent's observations is keyed by its name in the save file.
        if let Some(buffer) = data.observations_by_observer_name.remove(agent_name) {
            // Previous observations that reference entity IDs will have odd appearances in world inspectors because the IDs have been reused from restarts.

            *agent_buffer = buffer;

            let event = SomethingObservableHappenedEvent::MemoryRestored {
                observation_buffer_id: agent_id,
            };
            debug!("Sending event {:?}", event);
            observation_events.send(event);
        }
    }
    Ok(RestoreSuccess::Performed)
}
