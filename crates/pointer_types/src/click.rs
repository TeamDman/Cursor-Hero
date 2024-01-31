use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub struct Clickable;

#[derive(Reflect, Debug)]
pub struct PointerPress {
    pub pointer_id: Entity,
    pub way: Way,
}
#[derive(Component, Reflect, Debug)]
pub struct Pressed {
    pub presses: Vec<PointerPress>,
}

#[derive(Reflect, Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct TargetPress {
    pub target_id: Entity,
    pub way: Way,
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
        pointer_id: Entity,
        way: Way,
    },
    Released {
        target_id: Entity,
        pointer_id: Entity,
        way: Way,
    },
    Clicked {
        target_id: Entity,
        pointer_id: Entity,
        way: Way,
    },
}
#[derive(Event, Debug, Reflect)]
pub enum ToolClickEvent {
    Pressed { pointer_id: Entity, way: Way },
    Released { pointer_id: Entity, way: Way },
}
