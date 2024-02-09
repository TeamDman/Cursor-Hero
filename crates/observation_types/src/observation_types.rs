use bevy::{prelude::*, utils::Instant};

#[derive(Component, Reflect, Default)]
pub struct ObservationTool;

#[derive(Component, Reflect, Default)]
pub struct ObservationTimeline {
    pub observations: Vec<(Instant, Observation)>,
}

/// An observation must contain enough information for the transformation step 
/// where it is turned into a prompt for an LLM to generate a response.
/// 
/// Additional information is fine.
#[derive(Reflect, Clone, PartialEq, Debug)]
pub enum Observation {
    Chat {
        environment_id: Option<Entity>,
        character_id: Entity,
        character_name: String,
        message: String,
    },
}

#[derive(Event, Debug, Clone)]
pub enum ObservationEvent {
    ObservationToolResponse {
        character_id: Entity,
        observation: String,
    },
    SomethingHappened {
        observation: Observation,
    }
}
