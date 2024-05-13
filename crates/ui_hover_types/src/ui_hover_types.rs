use bevy::prelude::*;
use cursor_hero_ui_automation_types::prelude::ElementInfo;
use cursor_hero_worker_types::prelude::WorkerMessage;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct HoverInfo {
    pub host_hover_indicator: Option<HostHoverIndicator>,
    pub game_hover_indicator: Option<GameHoverIndicator>,
    pub inspector_hover_indicator: Option<InspectorHoverIndicator>,
    pub enabled: bool,
}
pub trait HoverIndicator {
    fn get_info(&self) -> &ElementInfo;
    fn get_bounds(&self) -> Rect;
}
#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct HostHoverIndicator {
    pub info: ElementInfo,
    pub cursor_pos: IVec2,
}
impl HoverIndicator for HostHoverIndicator {
    fn get_info(&self) -> &ElementInfo {
        &self.info
    }
    fn get_bounds(&self) -> Rect {
        self.info.bounding_rect.as_rect()
    }
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct GameHoverIndicator {
    pub info: ElementInfo,
    pub cursor_pos: IVec2,
}
impl HoverIndicator for GameHoverIndicator {
    fn get_info(&self) -> &ElementInfo {
        &self.info
    }
    fn get_bounds(&self) -> Rect {
        self.info.bounding_rect.as_rect()
    }
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct InspectorHoverIndicator {
    pub info: ElementInfo,
}
impl HoverIndicator for InspectorHoverIndicator {
    fn get_info(&self) -> &ElementInfo {
        &self.info
    }
    fn get_bounds(&self) -> Rect {
        self.info.bounding_rect.as_rect()
    }
}

#[derive(Debug, Reflect, Clone, Event)]
pub enum GameboundHoverMessage {
    HostHoverInfo {
        info: ElementInfo,
        cursor_pos: IVec2,
    },
    ClearHostHoverInfo,
    GameHoverInfo {
        info: ElementInfo,
        cursor_pos: IVec2,
    },
    ClearGameHoverInfo,
}
impl WorkerMessage for GameboundHoverMessage {}

#[derive(Debug, Reflect, Clone, Event, Eq, PartialEq)]
pub enum ThreadboundHoverMessage {
    AtPositionFromGame(IVec2),
    AtHostCursorPosition,
}
impl WorkerMessage for ThreadboundHoverMessage {}
