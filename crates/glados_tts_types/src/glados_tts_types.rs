use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::Instant;

#[derive(Reflect, Resource, Default, Debug, Eq, PartialEq, Clone, Copy)]
pub enum GladosTtsStatus {
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
pub struct GladosTtsStatusButton {
    pub visual_state: GladosTtsStatusButtonVisualState,
}

#[derive(Debug, Reflect, Eq, PartialEq)]
pub enum GladosTtsStatusButtonVisualState {
    Default { status: GladosTtsStatus },
    Hovered { status: GladosTtsStatus },
    Pressed { status: GladosTtsStatus },
}
impl Default for GladosTtsStatusButtonVisualState {
    fn default() -> Self {
        GladosTtsStatusButtonVisualState::Default {
            status: GladosTtsStatus::Unknown,
        }
    }
}

#[derive(Event, Debug, Reflect)]
pub enum GladosTtsPingEvent {
    Ping,
    Pong { status: GladosTtsStatus },
}

#[derive(Event, Debug, Reflect)]
pub enum GladosTtsStatusEvent {
    Changed { new_value: GladosTtsStatus },
    Startup,
}
