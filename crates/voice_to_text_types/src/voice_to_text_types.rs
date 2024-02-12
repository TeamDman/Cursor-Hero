use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::Instant;

#[derive(Reflect, Resource, Default, Debug, Eq, PartialEq, Clone, Copy)]
pub enum VoiceToTextStatus {
    #[default]
    Unknown,
    Alive,
    Dead,
    Starting {
        instant: Instant,
        timeout: Duration,
    },
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

