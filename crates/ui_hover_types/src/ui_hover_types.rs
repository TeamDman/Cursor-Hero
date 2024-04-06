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

#[derive(Component, Reflect, Debug, Clone)]
pub struct HostHoverIndicator {
    pub info: ElementInfo,
    pub cursor_pos: IVec2,
}
#[derive(Component, Reflect, Debug, Clone)]
pub struct GameHoverIndicator {
    pub info: ElementInfo,
    pub cursor_pos: IVec2,
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct InspectorHoverIndicator {
    pub info: ElementInfo,
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
    ClearHost,
    ClearGame,
}
impl WorkerMessage for ThreadboundHoverMessage {}
