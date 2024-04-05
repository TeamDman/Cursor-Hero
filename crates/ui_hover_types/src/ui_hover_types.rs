use bevy::prelude::*;
use cursor_hero_ui_automation_types::prelude::ElementInfo;
use cursor_hero_worker_types::prelude::WorkerMessage;

#[derive(Resource, Default, Reflect)]
pub struct HoverInfo {
    pub host_element: Option<ElementInfo>,
    pub game_element: Option<ElementInfo>,
    pub enabled: bool,
}

#[derive(Component, Reflect)]
pub struct HoveredElement {
    pub info: ElementInfo,
}



#[derive(Component, Reflect, Debug)]
pub struct ScreenHoveredIndicatorTag;
#[derive(Component, Reflect, Debug)]
pub struct GameHoveredIndicatorTag;



#[derive(Debug, Reflect, Clone, Event)]
pub enum GameboundHoverMessage {
    HostHoverInfo(ElementInfo),
    ClearHostHoverInfo,
    GameHoverInfo(ElementInfo),
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
