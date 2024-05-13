use bevy::prelude::*;
use cursor_hero_calculator_app_types::calculator_app_types::Calculator;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorState;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorThemeKind;
use cursor_hero_calculator_app_types::calculator_app_types::SpawnCalculatorRequestEvent;
use cursor_hero_environment_types::environment_types::AgentEnvironment;
use cursor_hero_memory_types::prelude::*;
use serde::Deserialize;
use serde::Serialize;

pub struct AppMemoryPlugin;

impl Plugin for AppMemoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AppMemoryConfig::default());
        app.add_systems(Update, persist.pipe(handle_persist_errors));
        app.add_systems(Update, restore.pipe(handle_restore_errors));
    }
}
const PERSIST_FILE_NAME: &str = "apps.json";

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
struct AppMemoryConfig {
    pub debounce_timer: Timer,
}
impl Default for AppMemoryConfig {
    fn default() -> Self {
        Self {
            debounce_timer: Timer::from_seconds(10.0, TimerMode::Repeating),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct DiskData {
    calculator_positions: Vec<Vec2>,
}

fn persist(
    mut config: ResMut<AppMemoryConfig>,
    memory_config: Res<MemoryConfig>,
    mut debounce: Local<Option<DiskData>>,
    time: Res<Time>,
    calculator_query: Query<&Transform, With<Calculator>>,
) -> Result<PersistSuccess, PersistError> {
    if !config.debounce_timer.tick(time.delta()).just_finished() {
        return Ok(PersistSuccess::Cooldown);
    }

    let mut calculator_positions = vec![];
    for transform in calculator_query.iter() {
        calculator_positions.push(transform.translation.xy());
    }
    let data = DiskData {
        calculator_positions,
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
    mut calculator_spawn_events: EventWriter<SpawnCalculatorRequestEvent>,
    environment_query: Query<Entity, Added<AgentEnvironment>>,
) -> Result<RestoreSuccess, RestoreError> {
    if environment_query.is_empty() {
        return Ok(RestoreSuccess::NoAction);
    }

    let file = get_persist_file(memory_config.as_ref(), PERSIST_FILE_NAME, Usage::Restore)
        .map_err(RestoreError::Io)?;
    let data: DiskData = read_from_disk(file)?;

    for environment in environment_query.iter() {
        let environment_id = environment;
        info!("Restoring calculator into {environment_id:?}");

        for position in &data.calculator_positions {
            calculator_spawn_events.send(SpawnCalculatorRequestEvent {
                environment_id,
                state: CalculatorState::default(),
                theme: CalculatorThemeKind::WindowsDark,
                position: *position,
            });
        }
    }

    Ok(RestoreSuccess::Performed)
}
