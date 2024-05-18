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

#[derive(Debug, Reflect, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectorWindows {
    pub global_toggle: bool,
    pub world: bool,
    // pub world_collapsed: bool,
    pub state: bool,
    // pub state_collapsed: bool,
    pub tree: bool,
    pub tree_header_open: bool,
    pub properties: bool,
    pub properties_header_open: bool,
    pub scratch_pad: bool,
    pub scratch_pad_header_open: bool,
}
impl InspectorWindows {
    pub fn set_all(&mut self, value: bool) {
        self.global_toggle = value;
        self.world = value;
        self.state = value;
        self.tree = value;
        self.properties = value;
        self.scratch_pad = value;
    }
}
impl Default for InspectorWindows {
    fn default() -> Self {
        Self {
            global_toggle: false,
            world: true,
            state: true,
            tree: true,
            tree_header_open: true,
            properties: true,
            properties_header_open: true,
            scratch_pad: true,
            scratch_pad_header_open: true,
        }
    }
}

#[derive(Resource, Debug, Reflect, Default)]
#[reflect(Resource)]
pub struct UIData {
    pub windows: InspectorWindows,
    pub scratch_pad: String,
    pub scratch_pad_mode: ScratchPadMode,
    pub mark: Option<DrillId>,
    pub start: ElementInfo,
    pub hovered: Option<ElementInfo>,
    pub tree: ElementInfo,
    pub selected: Option<DrillId>,
    pub selected_preview: Option<PreviewImage>,
    pub default_expanded: Vec<DrillId>,
    pub tree_is_fresh: bool,
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
    ScratchPadAppendAllUnknown,
}
