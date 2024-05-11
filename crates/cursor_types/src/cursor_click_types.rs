use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub struct Clickable;

#[derive(Reflect, Debug)]
pub struct CursorPress {
    pub cursor_id: Entity,
    pub way: Way,
    pub start_position: IVec2,
}
#[derive(Component, Reflect, Debug)]
pub struct Pressed {
    pub presses: Vec<CursorPress>,
}

#[derive(Reflect, Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct TargetPress {
    pub target_id: Entity,
    pub way: Way,
    pub start_position: IVec2,
}
#[derive(Component, Reflect, Debug)]
pub struct Pressing {
    pub pressing: Vec<TargetPress>,
}

#[derive(Reflect, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Way {
    Left,
    Right,
    Middle,
}

#[derive(Event, Debug, Reflect)]
pub enum ClickEvent {
    Pressed {
        target_id: Entity,
        cursor_id: Entity,
        start_position: IVec2,
        way: Way,
    },
    Released {
        target_id: Entity,
        cursor_id: Entity,
        start_position: IVec2,
        end_position: IVec2,
        way: Way,
    },
    Clicked {
        target_id: Entity,
        cursor_id: Entity,
        start_position: IVec2,
        end_position: IVec2,
        way: Way,
    },
}
#[derive(Event, Debug, Reflect)]
pub enum ToolClickEvent {
    Pressed { cursor_id: Entity, way: Way },
    Released { cursor_id: Entity, way: Way },
}
