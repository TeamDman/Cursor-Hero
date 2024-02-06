use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct ObservationTool;

#[derive(Event, Debug, Clone)]
pub enum ObservationEvent {
    ObservationToolResponse {
        character_id: Entity,
        observation: String,
    },
}
