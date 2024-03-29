use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::Instant;
use cursor_hero_ollama_types::prelude::*;

pub struct OllamaStatusPlugin;

impl Plugin for OllamaStatusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OllamaStatus>();
        app.add_systems(Update, handle_startup_event);
        app.add_systems(Update, periodic_ping);
        app.add_systems(Update, handle_pong);
    }
}

fn handle_startup_event(
    mut status_events: ParamSet<(
        EventReader<OllamaStatusEvent>,
        EventWriter<OllamaStatusEvent>,
    )>,
    mut ollama_status: ResMut<OllamaStatus>,
) {
    let starting = status_events
        .p0()
        .read()
        .any(|event| matches!(event, OllamaStatusEvent::Startup));
    if !starting {
        return;
    }
    status_events.p0().clear();

    *ollama_status = OllamaStatus::Starting {
        instant: Instant::now(),
        timeout: Duration::from_secs(60),
    };
    let event = OllamaStatusEvent::Changed {
        new_value: *ollama_status,
    };
    debug!("Sending event {:?}", event);
    status_events.p1().send(event);
}

fn periodic_ping(
    mut ping_events: EventWriter<OllamaPingEvent>,
    mut last_ping: Local<Option<Instant>>,
) {
    if let Some(instant) = *last_ping {
        if instant.elapsed().as_secs() > 5 {
            ping_events.send(OllamaPingEvent::Ping);
            *last_ping = Some(Instant::now());
        }
    } else {
        ping_events.send(OllamaPingEvent::Ping);
        *last_ping = Some(Instant::now());
    }
}

fn handle_pong(
    mut ping_events: EventReader<OllamaPingEvent>,
    mut status_events: EventWriter<OllamaStatusEvent>,
    mut ollama_status: ResMut<OllamaStatus>,
) {
    for event in ping_events.read() {
        let OllamaPingEvent::Pong { status } = event else {
            continue;
        };
        // identify the new state based on the pong
        let new_status = match (*ollama_status, *status) {
            // if starting, only change to dead if the timeout has been exceeded
            (OllamaStatus::Starting { instant, timeout }, status) => {
                if status == OllamaStatus::Alive {
                    OllamaStatus::Alive
                } else if instant.elapsed() > timeout {
                    OllamaStatus::Dead
                } else {
                    OllamaStatus::Starting { instant, timeout }
                }
            }
            // respect the new status if it's not starting
            _ => *status,
        };

        if *ollama_status != new_status {
            *ollama_status = new_status;
            let event = OllamaStatusEvent::Changed {
                new_value: new_status,
            };
            debug!("Sending event {:?}", event);
            status_events.send(event);
        }
    }
}
