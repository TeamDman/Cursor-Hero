use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::time::Duration;

use bevy::prelude::*;
use chrono::DateTime;
use chrono::Local;
use serde::Deserialize;
use serde::Serialize;
#[derive(Component, Reflect, Default)]
pub struct ObservationTool {
    #[reflect(ignore)]
    pub last_inference: Option<DateTime<Local>>,
    pub _whats_new: Option<WhatsNew>, // latest value for visual inspection
}

#[derive(Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum WhatsNew {
    // When the agent replies, it sends a chat, which becomes its own observation
    // Letting this trigger the inference again is a loop
    // We want to allow this loop, but only after a longer period of inactivity compared
    // to if a chat was received from another entity.
    SelfChat,
    Nothing,
    ChatReceived,
    ChatReceivedButTheyProbablyStillThinking,
    MemoryRestored,
}

impl WhatsNew {
    pub fn reply_delay(&self) -> Duration {
        match self {
            WhatsNew::SelfChat => Duration::from_secs(60),
            WhatsNew::Nothing => Duration::MAX,
            WhatsNew::ChatReceived => Duration::ZERO,
            WhatsNew::ChatReceivedButTheyProbablyStillThinking => Duration::from_secs(25),
            WhatsNew::MemoryRestored => Duration::from_secs(5),
        }
    }
}

impl Ord for WhatsNew {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match match (self, other) {
            (a, b) if a == b => std::cmp::Ordering::Equal,
            (WhatsNew::ChatReceived, _) => std::cmp::Ordering::Greater,
            (_, WhatsNew::ChatReceived) => std::cmp::Ordering::Less,

            (WhatsNew::Nothing, _) => std::cmp::Ordering::Less,
            (_, WhatsNew::Nothing) => std::cmp::Ordering::Greater,

            _ => std::cmp::Ordering::Equal,
        } {
            // If equal, lower delay takes priority
            std::cmp::Ordering::Equal => self.reply_delay().cmp(&other.reply_delay()).reverse(),
            x => x,
        }
    }
}
impl PartialOrd for WhatsNew {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Component, Reflect, Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ObservationBuffer {
    pub observations: Vec<ObservationBufferEntry>,
    pub log_level: ObservationLogLevel, // TODO: investigate always logging but updating the log filter instead of not logging based on level
}

#[derive(Debug, Reflect, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ObservationLogLevel {
    #[default]
    Default,
    All,
}

#[derive(Component, Reflect, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ObservationBufferEntry {
    #[reflect(ignore)]
    pub datetime: DateTime<Local>,
    pub origin: ObservationEvent,
}

#[derive(Event, Debug, Clone, Reflect)]
pub enum ObservationBufferEvent {
    Updated { buffer_id: Entity },
}

#[derive(Event, Debug, Clone, Reflect, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservationEvent {
    Chat {
        environment_id: Option<Entity>,
        character_id: Entity,
        character_name: String,
        message: String,
    },
    MemoryRestored {
        observation_buffer_id: Entity,
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
            ObservationEvent::MemoryRestored { .. } => {
                write!(
                    f,
                    "The game has restarted and the agent memory has been restored."
                )
            }
        }
    }
}
impl ObservationEvent {
    pub fn into_whats_new(&self, observation_buffer_id: Entity) -> WhatsNew {
        match self {
            ObservationEvent::Chat {
                character_id: event_character_id,
                ..
            } if *event_character_id == observation_buffer_id => WhatsNew::SelfChat,
            ObservationEvent::Chat { message, .. }
                if message.ends_with("...")
                    || !message.ends_with('.')
                    || !message.ends_with('!')
                    || !message.ends_with('?') =>
            {
                WhatsNew::ChatReceivedButTheyProbablyStillThinking
            }
            ObservationEvent::Chat { .. } => WhatsNew::ChatReceived,
            ObservationEvent::MemoryRestored { .. } => WhatsNew::MemoryRestored,
        }
    }
}
