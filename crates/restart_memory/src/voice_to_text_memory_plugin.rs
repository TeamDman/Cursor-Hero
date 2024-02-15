use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::Instant;
use cursor_hero_character_types::prelude::*;

use cursor_hero_secret_types::secrets_types::SecretString;
use cursor_hero_voice_to_text_types::voice_to_text_types::VoiceToTextStatus;
use cursor_hero_voice_to_text_types::voice_to_text_types::VoiceToTextStatusEvent;
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

pub struct VoiceToTextMemoryPlugin;

impl Plugin for VoiceToTextMemoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VoiceToTextMemoryConfig::default());
        app.add_systems(Update, persist.pipe(handle_persist_errors));
        app.add_systems(Update, restore.pipe(handle_restore_errors));
    }
}
const PERSIST_FILE_NAME: &str = "voice_to_text.json";

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
struct VoiceToTextMemoryConfig {
    pub persist_cooldown: Timer,
    pub restore_retry_cooldown: Duration,
}
impl Default for VoiceToTextMemoryConfig {
    fn default() -> Self {
        Self {
            persist_cooldown: Timer::from_seconds(10.0, TimerMode::Repeating),
            restore_retry_cooldown: Duration::from_secs(60),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct DiskData {
    api_key: Option<SecretString>,
}

fn persist(
    mut config: ResMut<VoiceToTextMemoryConfig>,
    mut debounce: Local<Option<DiskData>>,
    time: Res<Time>,
    voice_status: Res<VoiceToTextStatus>,
) -> Result<PersistSuccess, PersistError> {
    if !config.persist_cooldown.tick(time.delta()).just_finished() {
        return Ok(PersistSuccess::Cooldown);
    }
    let api_key = match &*voice_status {
        VoiceToTextStatus::Alive { api_key, .. } | VoiceToTextStatus::Starting { api_key, .. } => {
            Some(api_key.clone())
        }
        VoiceToTextStatus::Dead => {
            None
        }
        _ => {
            return Ok(PersistSuccess::NoAction);
        }
    };
    let data = DiskData { api_key };
    if debounce.is_none() || debounce.as_ref().unwrap() != &data {
        let file = get_persist_file(file!(), PERSIST_FILE_NAME, Usage::Persist)
            .map_err(PersistError::Io)?;
        write_to_disk(file, data.clone())?;
        *debounce = Some(data);
        Ok(PersistSuccess::WritePerformed)
    } else {
        Ok(PersistSuccess::Debounce)
    }
}

fn restore(
    config: Res<VoiceToTextMemoryConfig>,
    mut current_status: ResMut<VoiceToTextStatus>,
    mut status_events: EventWriter<VoiceToTextStatusEvent>,
    mut attempted_at: Local<Option<Instant>>,
) -> Result<RestoreSuccess, RestoreError> {
    if matches!(
        *current_status,
        VoiceToTextStatus::Alive { .. }
            | VoiceToTextStatus::Starting { .. }
            | VoiceToTextStatus::UnknownWithCachedApiKey { .. }
            | VoiceToTextStatus::Dead
    ) {
        return Ok(RestoreSuccess::NoAction);
    }
    let file = match get_persist_file(file!(), PERSIST_FILE_NAME, Usage::Restore) {
        Ok(file) => Ok(file),
        Err(e) => {
            if let Some(attempt) = *attempted_at {
                if attempt.elapsed() > config.restore_retry_cooldown {
                    *attempted_at = Some(Instant::now());
                    return Err(RestoreError::Io(e));
                } else {
                    // Silently ignore the error and retry later
                    return Ok(RestoreSuccess::NoAction);
                }
            } else {
                *attempted_at = Some(Instant::now());
                return Err(RestoreError::Io(e));
            }
        }
    }?;
    let data: DiskData = read_from_disk(file)?;
    let Some(api_key) = data.api_key else {
        return Ok(RestoreSuccess::NoAction);
    };
    
    info!("Restoring api key");

    let new_status = match *current_status {
        VoiceToTextStatus::Unknown
        | VoiceToTextStatus::AliveButWeDontKnowTheApiKey
        | VoiceToTextStatus::UnknownWithCachedApiKey { .. } => {
            VoiceToTextStatus::UnknownWithCachedApiKey {
                api_key,
            }
        }
        ref current => current.clone(),
    };
    if new_status != *current_status {
        let event = VoiceToTextStatusEvent::Changed {
            old_status: current_status.clone(),
            new_status: new_status.clone(),
        };
        debug!("Sending event: {:?}", event);
        status_events.send(event);
        *current_status = new_status;
        Ok(RestoreSuccess::Performed)
    } else {
        Ok(RestoreSuccess::NoAction)
    }
}
