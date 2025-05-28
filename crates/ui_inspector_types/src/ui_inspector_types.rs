use std::marker::PhantomData;

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
    MapText,
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
            Self::MapText,
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
            Self::MapText => write!(f, "Map Text"),
        }
    }
}

#[derive(Debug, Reflect, Serialize, Deserialize, PartialEq, Clone)]
pub struct EguiWindow {
    pub open: bool,
    pub header_open: bool,
    pub position: Option<IVec2>,
    pub size: Option<IVec2>,
}
impl Default for EguiWindow {
    fn default() -> Self {
        Self {
            open: true,
            header_open: true,
            position: None,
            size: None,
        }
    }
}

#[derive(Debug, Reflect, Serialize, Deserialize, PartialEq, Clone)]
pub struct InspectorWindows {
    pub global_toggle: bool,
    pub world: EguiWindow,
    pub state: EguiWindow,
    pub tree: EguiWindow,
    pub properties: EguiWindow,
    pub scratch_pad: EguiWindow,
}

pub struct InspectorWindowsIter<'a> {
    windows: &'a InspectorWindows,
    index: usize,
}
pub struct InspectorWindowsIterMut<'a> {
    windows: *mut InspectorWindows,
    index: usize,
    _marker: PhantomData<&'a mut EguiWindow>,
}

impl InspectorWindows {
    pub fn iter(&self) -> InspectorWindowsIter {
        InspectorWindowsIter {
            windows: self,
            index: 0,
        }
    }
    pub fn iter_mut(&mut self) -> InspectorWindowsIterMut {
        InspectorWindowsIterMut {
            windows: self,
            index: 0,
            _marker: PhantomData,
        }
    }

}

impl<'a> Iterator for InspectorWindowsIter<'a> {
    type Item = &'a EguiWindow;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(&self.windows.world),
            1 => Some(&self.windows.state),
            2 => Some(&self.windows.tree),
            3 => Some(&self.windows.properties),
            4 => Some(&self.windows.scratch_pad),
            _ => None,
        };
        self.index += 1;
        result
    }
}

impl<'a> Iterator for InspectorWindowsIterMut<'a> {
    type Item = &'a mut EguiWindow;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let result = match self.index {
                0 => Some(&mut (*self.windows).world),
                1 => Some(&mut (*self.windows).state),
                2 => Some(&mut (*self.windows).tree),
                3 => Some(&mut (*self.windows).properties),
                4 => Some(&mut (*self.windows).scratch_pad),
                _ => None,
            };
            self.index += 1;
            result
        }
    }
}

impl Default for InspectorWindows {
    fn default() -> Self {
        Self {
            global_toggle: false,
            world: EguiWindow::default(),
            state: EguiWindow::default(),
            tree: EguiWindow::default(),
            properties: EguiWindow::default(),
            scratch_pad: EguiWindow::default(),
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
