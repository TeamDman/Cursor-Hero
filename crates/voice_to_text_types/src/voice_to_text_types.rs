use std::fmt::Debug;
use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::Instant;

#[derive(Reflect, Resource, Default, Eq, PartialEq, Clone)]
#[reflect(Resource)]
pub enum VoiceToTextStatus {
    #[default]
    Unknown,
    Alive {
        api_key: String,
        listening: bool,
    },
    AliveButWeDontKnowTheApiKey,
    Dead,
    Starting {
        instant: Instant,
        timeout: Duration,
        api_key: String,
    },
}
impl Debug for VoiceToTextStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoiceToTextStatus::Unknown => write!(f, "Unknown"),
            VoiceToTextStatus::Alive { api_key, listening } => {
                write!(
                    f,
                    "Alive {{ api_key: {}, listening: {} }}",
                    match api_key.len() {
                        0 => "<empty>".to_string(),
                        _ => "<redacted>".to_string(),
                    },
                    listening
                )
            }
            VoiceToTextStatus::AliveButWeDontKnowTheApiKey => {
                write!(f, "AliveButWeDontKnowTheApiKey")
            }
            VoiceToTextStatus::Dead => write!(f, "Dead"),
            VoiceToTextStatus::Starting {
                instant,
                timeout,
                api_key,
            } => write!(
                f,
                "Starting {{ instant: {:?}, timeout: {:?}, api_key: {} }}",
                instant, timeout, api_key
            ),
        }
    }
}

#[derive(Event, Debug, Reflect)]
pub enum VoiceToTextPingEvent {
    Ping,
    Pong { status: VoiceToTextStatus },
}

#[derive(Event, Debug, Reflect)]
pub enum VoiceToTextStatusEvent {
    Changed { new_value: VoiceToTextStatus },
    Startup,
}
#[derive(Event, Debug, Reflect)]
pub enum VoiceToTextCommandEvent {
    Startup,
    SetListening { listening: bool, api_key: String },
}

#[derive(Component, Debug, Reflect, Default)]
pub struct VoiceToTextStatusButton {
    pub visual_state: VoiceToTextStatusButtonVisualState,
}

#[derive(Debug, Reflect, Eq, PartialEq)]
pub enum VoiceToTextStatusButtonVisualState {
    Default { status: VoiceToTextStatus },
    Hovered { status: VoiceToTextStatus },
    Pressed { status: VoiceToTextStatus },
}
impl Default for VoiceToTextStatusButtonVisualState {
    fn default() -> Self {
        VoiceToTextStatusButtonVisualState::Default {
            status: VoiceToTextStatus::Unknown,
        }
    }
}

#[derive(Component, Debug, Reflect, Default)]
pub struct VoiceToTextVscodeButton {
    pub visual_state: VoiceToTextVscodeButtonVisualState,
}
#[derive(Debug, Reflect, Eq, PartialEq, Default)]
pub enum VoiceToTextVscodeButtonVisualState {
    #[default]
    Default,
}
#[derive(Event, Debug, Reflect)]
pub enum VoiceToTextVscodeEvent {
    Startup,
}
