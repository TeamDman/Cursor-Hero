use bevy::prelude::*;
use cursor_hero_memory_types::prelude::*;
use cursor_hero_ui_automation_types::prelude::DrillId;
use cursor_hero_ui_automation_types::prelude::ElementInfo;
use cursor_hero_ui_inspector_types::prelude::ScratchPadMode;
use cursor_hero_ui_inspector_types::prelude::UIData;
use serde::Deserialize;
use serde::Serialize;

pub struct UIDataMemoryPlugin;

impl Plugin for UIDataMemoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIDataMemoryConfig::default());
        app.add_systems(Update, persist.pipe(handle_persist_errors));
        app.add_systems(
            Startup,
            (apply_deferred, restore.pipe(handle_restore_errors)).chain(),
        );
    }
}
const PERSIST_FILE_NAME: &str = "ui_data.json";

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
struct UIDataMemoryConfig {
    pub debounce_timer: Timer,
}
impl Default for UIDataMemoryConfig {
    fn default() -> Self {
        Self {
            debounce_timer: Timer::from_seconds(10.0, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct DiskData {
    pub visible: bool,
    pub open: bool,
    pub scratch_pad: String,
    pub scratch_pad_mode: ScratchPadMode,
    pub mark: Option<DrillId>,
    pub start: ElementInfo,
    pub hovered: Option<ElementInfo>,
    pub ui_tree: ElementInfo,
    pub selected: Option<DrillId>,
    pub expanded: Vec<DrillId>,
    pub paused: bool,
}
impl From<DiskData> for UIData {
    fn from(value: DiskData) -> Self {
        UIData {
            visible: value.visible,
            open: value.open,
            scratch_pad: value.scratch_pad,
            scratch_pad_mode: value.scratch_pad_mode,
            mark: value.mark,
            start: value.start,
            hovered: value.hovered,
            ui_tree: value.ui_tree,
            selected: value.selected,
            expanded: value.expanded,
            paused: value.paused,
            ..default()
        }
    }
}
impl From<&UIData> for DiskData {
    fn from(value: &UIData) -> Self {
        Self {
            open: value.open,
            visible: value.visible,
            scratch_pad: value.scratch_pad.clone(),
            scratch_pad_mode: value.scratch_pad_mode.clone(),
            mark: value.mark.clone(),
            start: value.start.clone(),
            hovered: value.hovered.clone(),
            ui_tree: value.ui_tree.clone(),
            selected: value.selected.clone(),
            expanded: value.expanded.clone(),
            paused: value.paused,
        }
    }
}

fn persist(
    mut config: ResMut<UIDataMemoryConfig>,
    memory_config: Res<MemoryConfig>,
    mut debounce: Local<Option<DiskData>>,
    time: Res<Time>,
    ui_data: Res<UIData>,
) -> Result<PersistSuccess, PersistError> {
    if !config.debounce_timer.tick(time.delta()).just_finished() {
        return Ok(PersistSuccess::Cooldown);
    }

    let data = ui_data.as_ref().into();
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
    mut ui_data: ResMut<UIData>,
) -> Result<RestoreSuccess, RestoreError> {
    let file = get_persist_file(memory_config.as_ref(), PERSIST_FILE_NAME, Usage::Restore)
        .map_err(RestoreError::Io)?;
    let mut data: DiskData = read_from_disk(file)?;

    info!("Restoring UI Data");

    // other debug systems are hidden by default
    // force this to be invisible at start until a global debug state is implemented
    data.visible = false;

    *ui_data = data.into();

    Ok(RestoreSuccess::Performed)
}
