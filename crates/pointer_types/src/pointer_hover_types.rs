use bevy::prelude::*;

#[derive(Event, Debug, Reflect)]
pub enum HoverEvent {
    Start {
        target_id: Entity,
        pointer_id: Entity,
    },
    End {
        target_id: Entity,
        pointer_id: Entity,
    },
}

#[derive(Component, Reflect, Debug)]
pub struct Hovered;
#[derive(Component, Reflect, Debug)]
pub struct Hoverable;
#[derive(Component, Reflect, Debug)]
pub struct Hovering {
    pub hovering: Vec<Entity>,
}
