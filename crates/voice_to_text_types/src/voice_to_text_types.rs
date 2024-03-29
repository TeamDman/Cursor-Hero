use std::fmt::Debug;
use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::Instant;
use cursor_hero_secret_types::prelude::*;

#[derive(Reflect, Resource, Default, Debug, Clone, Eq, PartialEq)]
#[reflect(Resource)]
pub enum VoiceToTextStatus {
    #[default]
    Unknown,
    UnknownWithCachedApiKey {
        api_key: SecretString,
    },
    Alive {
        api_key: SecretString,
        listening: bool,
    },
    AliveButWeDontKnowTheApiKey,
    Dead,
    Starting {
        instant: Instant,
        timeout: Duration,
        api_key: SecretString,
    },
}

#[derive(Event, Debug, Reflect)]
pub enum VoiceToTextPingEvent {
    Ping,
    Pong { status: VoiceToTextStatus },
}

#[derive(Event, Debug, Reflect)]
pub enum VoiceToTextStatusEvent {
    Changed {
        old_status: VoiceToTextStatus,
        new_status: VoiceToTextStatus,
    },
}

#[derive(Event, Debug, Reflect)]
pub enum VoiceToTextTranscriptionEvent {
    Received { transcription: String },
}

#[derive(Event, Debug, Reflect)]
pub enum VoiceToTextCommandEvent {
    Startup,
    SetListening {
        listening: bool,
        api_key: SecretString,
    },
    ConnectReceiver {
        api_key: SecretString,
    },
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
