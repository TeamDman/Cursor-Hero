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
    if starting {
        *ollama_status = OllamaStatus::Starting {
            instant: Instant::now(),
            timeout: Duration::from_secs(5),
        };
        let event = OllamaStatusEvent::Changed {
            new_value: *ollama_status,
        };
        debug!("Sending event {:?}", event);
        status_events.p1().send(event);
    }
}

fn periodic_ping(
    ollama_status: Res<OllamaStatus>,
    mut ping_events: EventWriter<OllamaPingEvent>,
    mut last_ping: Local<Option<Instant>>,
) {
    let (OllamaStatus::Starting { .. } | OllamaStatus::Unknown) = *ollama_status else {
        return;
    };
    if let Some(instant) = *last_ping {
        if instant.elapsed().as_secs() > 2 {
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
        match event {
            OllamaPingEvent::Pong { status } => {
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
                    let event = OllamaStatusEvent::Changed { new_value: new_status };
                    debug!("Sending event {:?}", event);
                    status_events.send(event);
                }
            }
            _ => {}
        }
    }
}

// fn startup_timeout(
//     mut ollama_status: ResMut<OllamaStatus>,
//     mut status_events: EventWriter<OllamaStatusEvent>,
// ) {
//     let OllamaStatus::Starting { instant, timeout } = *ollama_status else {
//         return;
//     };
//     if instant.elapsed() > timeout {
//         *ollama_status = OllamaStatus::Dead;
//         let event = OllamaStatusEvent::Changed {
//             new_value: OllamaStatus::Dead,
//         };
//         debug!("Startup timeout exceeded, sending event {:?}", event);
//         status_events.send(event);
//         return;
//     }
// }
