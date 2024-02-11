use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::Instant;
use chrono::DateTime;
use chrono::Local;

#[derive(Component, Reflect)]
pub struct ObservationTool {
    pub last_inference: Option<Instant>,
    pub _whats_new: Option<WhatsNew>,
}
impl Default for ObservationTool {
    fn default() -> Self {
        Self {
            last_inference: None,
            _whats_new: None,
        }
    }
}

#[derive(Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum WhatsNew {
    // When the agent replies, it sends a chat, which becomes its own observation
    // Letting this trigger the inference again is a loop
    // We want to allow this loop, but only after a longer period of inactivity compared
    // to if a chat was received from another entity.
    SelfChat,
    Nothing,
    Something,
}

impl WhatsNew {
    pub fn cooldown(&self) -> Duration {
        match self {
            WhatsNew::SelfChat => {
                Duration::from_secs(60)
            }
            WhatsNew::Nothing => {
                Duration::from_secs(120)
            }
            WhatsNew::Something => {
                Duration::ZERO
            }
        }
    }
}


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
                write!(f, "{}: {}", character_name, message)
            }
        }
    }
}
