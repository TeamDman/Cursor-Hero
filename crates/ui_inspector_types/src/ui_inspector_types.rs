use bevy::prelude::*;
use bevy::utils::HashMap;
use cursor_hero_ui_automation_types::prelude::DrillId;
use cursor_hero_ui_automation_types::prelude::ElementInfo;
use cursor_hero_ui_automation_types::prelude::RuntimeId;

#[derive(Resource, Debug, Reflect, Default)]
pub struct PreviewImage {
    pub handle: Handle<Image>,
    pub size: UVec2,
}

#[derive(Debug, Reflect, Default, Eq, PartialEq)]
pub enum ScratchPadMode {
    #[default]
    Drill,
    Bounds,
}

#[derive(Resource, Debug, Reflect, Default)]
#[reflect(Resource)]
pub struct UIData {
    pub scratch_pad: String,
    pub scratch_pad_mode: ScratchPadMode,
    pub mark: Option<DrillId>,
    pub start: ElementInfo,
    pub hovered: Option<ElementInfo>,
    pub ui_tree: ElementInfo,
    pub selected: Option<DrillId>,
    pub selected_preview: Option<PreviewImage>,
    pub expanded: Vec<DrillId>,
    pub fresh: bool,
    pub in_flight: bool,
    pub paused: bool,
    // Include runtime id in case tree changes and we quickly fetch something with the same drill_id before the first request comes back
    pub fetching: HashMap<(DrillId, RuntimeId), FetchingState>,
}

#[derive(Debug, Reflect)]
pub enum FetchingState {
    FetchRequest,
    FetchDispatched,
    Fetched(Vec<ElementInfo>),
}

#[derive(Debug, Reflect, Event)]
pub enum InspectorEvent {
    PushScratchPad,
}
