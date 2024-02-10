use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use bevy::prelude::*;
use bevy::utils::Instant;
use chrono::DateTime;
use chrono::Local;

#[derive(Component, Reflect, Default)]
pub struct ObservationTool;

#[derive(Component, Reflect, Default)]
pub struct ObservationBuffer {
    pub observations: Vec<ObservationBufferEntry>,
    pub log_level: ObservationLogLevel,
}


#[derive(Debug, Reflect, Default, PartialEq, Eq)]
pub enum ObservationLogLevel {
    #[default]
    Default,
    All,
}

#[derive(Component, Reflect, Debug)]
pub struct ObservationBufferEntry {
    pub instant: Instant,
    #[reflect(ignore)]
    pub datetime: DateTime<Local>,
    pub observation: String,
    pub origin: ObservationEvent,
}

#[derive(Event, Debug, Clone, Reflect)]
pub enum ObservationBufferEvent {
    Updated { buffer_id: Entity },
}

#[derive(Event, Debug, Clone, Reflect)]
pub enum ObservationEvent {
    Chat {
        environment_id: Option<Entity>,
        character_id: Entity,
        character_name: String,
        message: String,
    },
}
impl Display for ObservationEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ObservationEvent::Chat {
                character_name,
                message,
                ..
            } => {
                write!(f, "<{:?}> {:?}", character_name, message)
            }
        }
    }
}
