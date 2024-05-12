use bevy::prelude::*;
use bevy::utils::HashMap;
use cursor_hero_ui_automation_types::prelude::DrillId;
use cursor_hero_ui_automation_types::prelude::ElementInfo;
use cursor_hero_ui_automation_types::prelude::RuntimeId;
use serde::Deserialize;
use serde::Serialize;

#[derive(Resource, Debug, Reflect, Default, Clone)]
pub struct PreviewImage {
    pub handle: Handle<Image>,
    pub size: UVec2,
}

#[derive(Debug, Reflect, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum ScratchPadMode {
    #[default]
    Identify,
    MapIdentify,
    PerformDrill,
    MapDrill,
    MapBounds,
    MapColor,
}
impl ScratchPadMode {
    pub fn variants() -> Vec<Self> {
        vec![
            Self::Identify,
            Self::MapIdentify,
            Self::PerformDrill,
            Self::MapDrill,
            Self::MapBounds,
            Self::MapColor,
        ]
    }
}
impl std::fmt::Display for ScratchPadMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identify => write!(f, "Identify"),
            Self::MapIdentify => write!(f, "Map Identify"),
            Self::PerformDrill => write!(f, "Perform Drill"),
            Self::MapDrill => write!(f, "Map Drill"),
            Self::MapBounds => write!(f, "Map Bounds"),
            Self::MapColor => write!(f, "Map Color"),
        }
    }
}

#[derive(Resource, Debug, Reflect, Default)]
#[reflect(Resource)]
pub struct UIData {
    pub visible: bool,
    pub open: bool,
    pub scratch_pad: String,
    pub scratch_pad_mode: ScratchPadMode,
    pub mark: Option<DrillId>,
    pub start: ElementInfo,
    pub hovered: Option<ElementInfo>,
    pub ui_tree: ElementInfo,
    pub selected: Option<DrillId>,
    pub selected_preview: Option<PreviewImage>,
    pub default_expanded: Vec<DrillId>,
    pub fresh: bool,
    pub in_flight: bool,
    pub paused: bool,
    // Include runtime id in case tree changes and we quickly fetch something with the same drill_id before the first request comes back
    pub fetching: HashMap<(DrillId, RuntimeId), FetchingState>,
}

#[derive(Debug, Reflect, Clone)]
pub enum FetchingState {
    RequestingFetch,
    FetchDispatched,
    Fetched(Vec<ElementInfo>),
}

#[derive(Debug, Reflect, Event)]
pub enum InspectorScratchPadEvent {
    ScratchPadAppendInfo { info: ElementInfo },
    ScratchPadAppendAllKnown,
    ScratchPadAppendAll,
}
