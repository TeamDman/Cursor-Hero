use bevy::prelude::*;

#[derive(Event, Debug, Reflect, Eq, PartialEq)]
pub enum HostEvent {
    MousePhysicallyMoved,
}
