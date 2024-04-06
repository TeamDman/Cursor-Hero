use bevy::{prelude::*, utils::HashMap};
use cursor_hero_ui_automation_types::prelude::{DrillId, ElementInfo, RuntimeId};

#[derive(Resource, Debug, Reflect, Default)]
#[reflect(Resource)]
pub struct UIData {
    pub start: ElementInfo,
    pub hovered: Option<ElementInfo>,
    pub ui_tree: ElementInfo,
    pub selected: Option<DrillId>,
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
