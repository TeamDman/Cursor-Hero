use bevy::prelude::*;
use cursor_hero_cursor_types::prelude::Way;
use cursor_hero_ui_automation_types::prelude::*;

#[derive(Debug, Reflect, Clone, Event)]
pub enum ThreadboundUISnapshotMessage {
    TreeUpdate {
        pos: IVec2,
    },
    GatherChildren {
        parent_drill_id: DrillId,
        parent_runtime_id: RuntimeId,
    },
    TreeClipboard {
        parent_drill_id: DrillId,
        parent_runtime_id: RuntimeId,
    },
    TreePatch {
        parent_drill_id: DrillId,
        parent_runtime_id: RuntimeId,
    },
    Click {
        drill_id: DrillId,
        way: Way,
    },
    ClickPos {
        pos: IVec2,
        way: Way,
    },
}

#[derive(Debug, Reflect, Clone, Event)]
pub enum GameboundUISnapshotMessage {
    SetUITree {
        ui_tree: ElementInfo,
        start: ElementInfo,
    },
    PatchUITree {
        patch: ElementInfo,
    },
    GatherChildrenResponse {
        drill_id: DrillId,
        runtime_id: RuntimeId,
        children: Vec<ElementInfo>,
    },
    TreeClipboardResponse {
        ui_tree: ElementInfo,
    },
    Error,
}



