use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::time::Duration;

use bevy::prelude::*;
use chrono::DateTime;
use chrono::Local;
use cursor_hero_ui_automation_types::prelude::UiSnapshot;
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
    Nothing,
    SelfChat,
    ChatReceived,
    ChatReceivedButTheyProbablyStillThinking,
    MemoryRestored,
    UISnapshot,
}

impl WhatsNew {
    /// When the agent replies, it sends a chat, which becomes its own observation
    /// Letting this trigger the inference again is a loop
    /// We want to allow this loop, but only after a longer period of inactivity compared
    /// to if a chat was received from another entity.
    pub fn reply_delay(&self) -> Duration {
        match self {
            WhatsNew::SelfChat => Duration::from_secs(60),
            WhatsNew::Nothing => Duration::MAX,
            WhatsNew::ChatReceived => Duration::ZERO,
            WhatsNew::ChatReceivedButTheyProbablyStillThinking => Duration::from_secs(25),
            WhatsNew::MemoryRestored => Duration::from_secs(5),
            WhatsNew::UISnapshot => Duration::from_secs(60 * 2),
        }
    }
}

impl Ord for WhatsNew {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // In-declaration order of importance, higher is more important
        (*self as u32).cmp(&(*other as u32))
    }
}
impl PartialOrd for WhatsNew {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Component, Reflect, Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Component, Reflect, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ObservationBufferEntry {
    #[reflect(ignore)]
    pub datetime: DateTime<Local>,
    pub origin: SomethingObservableHappenedEvent,
}
impl std::fmt::Display for ObservationBufferEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.datetime, self.origin)
    }
}

#[derive(Event, Debug, Clone, Reflect)]
pub enum ObservationBufferEvent {
    Updated { buffer_id: Entity },
}

#[derive(Event, Debug, Clone, Reflect, Serialize, Deserialize, PartialEq)]
pub enum SomethingObservableHappenedEvent {
    Chat {
        environment_id: Option<Entity>,
        character_id: Entity,
        character_name: String,
        message: String,
    },
    MemoryRestored {
        observation_buffer_id: Entity,
    },
    UISnapshot {
        environment_id: Option<Entity>,
        snapshot: UiSnapshot,
    },
    // BrickEnteredEnvironment {

    // }
}
impl Display for SomethingObservableHappenedEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SomethingObservableHappenedEvent::Chat {
                character_name,
                message,
                ..
            } => {
                write!(f, "{}: {}", character_name, message)
            }
            SomethingObservableHappenedEvent::MemoryRestored { .. } => {
                write!(
                    f,
                    "The game has restarted and the agent memory has been restored."
                )
            }
            SomethingObservableHappenedEvent::UISnapshot { snapshot, .. } => {
                write!(f, "Snapshot with {} windows", snapshot.app_windows.len())
            }
        }
    }
}
impl SomethingObservableHappenedEvent {
    pub fn into_whats_new(&self, observation_buffer_id: Entity) -> WhatsNew {
        match self {
            SomethingObservableHappenedEvent::Chat {
                character_id: event_character_id,
                ..
            } if *event_character_id == observation_buffer_id => WhatsNew::SelfChat,
            SomethingObservableHappenedEvent::Chat { message, .. }
                if message.ends_with("...")
                    || !message.ends_with('.')
                        && !message.ends_with('!')
                        && !message.ends_with('?') =>
            {
                WhatsNew::ChatReceivedButTheyProbablyStillThinking
            }
            SomethingObservableHappenedEvent::Chat { .. } => WhatsNew::ChatReceived,
            SomethingObservableHappenedEvent::MemoryRestored { .. } => WhatsNew::MemoryRestored,
            SomethingObservableHappenedEvent::UISnapshot { .. } => WhatsNew::UISnapshot,
        }
    }
}
