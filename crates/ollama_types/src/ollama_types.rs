use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::Instant;

#[derive(Reflect, Resource, Default, Debug, Eq, PartialEq, Clone, Copy)]
#[reflect(Resource)]
pub enum OllamaStatus {
    #[default]
    Unknown,
    Alive,
    Dead,
    Starting {
        instant: Instant,
        timeout: Duration,
    },
}

#[derive(Component, Debug, Reflect, Default)]
pub struct OllamaStatusButton {
    pub visual_state: OllamaStatusButtonVisualState,
}

#[derive(Debug, Reflect, Eq, PartialEq)]
pub enum OllamaStatusButtonVisualState {
    Default { status: OllamaStatus },
    Hovered { status: OllamaStatus },
    Pressed { status: OllamaStatus },
}
impl Default for OllamaStatusButtonVisualState {
    fn default() -> Self {
        OllamaStatusButtonVisualState::Default {
            status: OllamaStatus::Unknown,
        }
    }
}

#[derive(Event, Debug, Reflect)]
pub enum OllamaPingEvent {
    Ping,
    Pong { status: OllamaStatus },
}

#[derive(Event, Debug, Reflect)]
pub enum OllamaStatusEvent {
    Changed { new_value: OllamaStatus },
    Startup,
}

#[derive(Component, Debug, Reflect, Default)]
pub struct OllamaTerminalButton {
    pub visual_state: OllamaTerminalButtonVisualState,
}
#[derive(Debug, Reflect, Eq, PartialEq, Default)]
pub enum OllamaTerminalButtonVisualState {
    #[default]
    Default,
}
#[derive(Event, Debug, Reflect)]
pub enum OllamaTerminalEvent {
    Startup,
}
