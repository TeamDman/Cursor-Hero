use bevy::prelude::*;
use bevy::utils::Instant;
use cursor_hero_voice_to_text_types::prelude::*;

pub struct VoiceToTextPingPlugin;

impl Plugin for VoiceToTextPingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, periodic_ping);
        app.add_systems(Update, handle_pong);
        app.add_systems(Update, init_receiver_once_alive);
    }
}

fn periodic_ping(
    mut ping_events: EventWriter<VoiceToTextPingEvent>,
    mut last_ping: Local<Option<Instant>>,
) {
    if let Some(instant) = *last_ping {
        if instant.elapsed().as_secs() > 5 {
            ping_events.send(VoiceToTextPingEvent::Ping);
            *last_ping = Some(Instant::now());
        }
    } else {
        ping_events.send(VoiceToTextPingEvent::Ping);
        *last_ping = Some(Instant::now());
    }
}

fn handle_pong(
    mut ping_events: EventReader<VoiceToTextPingEvent>,
    mut status_events: EventWriter<VoiceToTextStatusEvent>,
    mut current_status: ResMut<VoiceToTextStatus>,
) {
    for event in ping_events.read() {
        let VoiceToTextPingEvent::Pong { status: new_status } = event else {
            continue;
        };
        // identify the new state based on the pong
        // if the pong says dead and the current state is starting, only change to dead if the timeout has been exceeded
        let new_status = match (&*current_status, new_status) {
            (
                VoiceToTextStatus::Starting {
                    instant,
                    timeout,
                    api_key,
                },
                status,
            ) => {
                if let VoiceToTextStatus::Alive {
                    api_key: other_api_key,
                    listening,
                } = status
                {
                    // Unlikely branch, but lets be safe
                    if other_api_key != api_key {
                        warn!("Received pong with Alive status with an api key different from the one we tracked when starting the program, overwriting api key")
                    }
                    VoiceToTextStatus::Alive {
                        api_key: other_api_key.clone(),
                        listening: *listening,
                    }
                } else if *status == VoiceToTextStatus::AliveButWeDontKnowTheApiKey {
                    // A server has responded to our ping, assume the API key is the one we tracked when we started the program
                    VoiceToTextStatus::Alive {
                        api_key: api_key.clone(),
                        listening: false,
                    }
                } else if instant.elapsed() > *timeout {
                    // Only accept the dead status if the timeout has been exceeded
                    VoiceToTextStatus::Dead
                } else {
                    // Timeout not exceeded, keep the current status (starting)
                    current_status.clone()
                }
            }
            (VoiceToTextStatus::Alive { .. }, VoiceToTextStatus::AliveButWeDontKnowTheApiKey) => {
                // Ping is alive, retain the api key
                current_status.clone()
            }
            (
                VoiceToTextStatus::UnknownWithCachedApiKey { api_key },
                VoiceToTextStatus::AliveButWeDontKnowTheApiKey,
            ) => {
                // Ping is alive, use the cached api key
                VoiceToTextStatus::Alive {
                    api_key: api_key.clone(),
                    listening: false,
                }
            }
            (a, b) => {
                debug!(
                    "Received pong with status {:?} but the current status is {:?}",
                    b, a
                );
                new_status.clone()
            }
        };

        if *current_status != new_status {
            let event = VoiceToTextStatusEvent::Changed {
                old_status: current_status.clone(),
                new_status: new_status.clone(),
            };
            debug!("Sending event {:?}", event);
            status_events.send(event);

            *current_status = new_status;
        }
    }
}

fn init_receiver_once_alive(
    mut command_events: EventWriter<VoiceToTextCommandEvent>,
    mut status_events: EventReader<VoiceToTextStatusEvent>,
) {
    for event in status_events.read() {
        let VoiceToTextStatusEvent::Changed {
            new_status,
            old_status,
        } = event;
        if !matches!(
            old_status,
            VoiceToTextStatus::Starting { .. } | VoiceToTextStatus::UnknownWithCachedApiKey { .. }
        ) {
            continue;
        }
        let VoiceToTextStatus::Alive { api_key, .. } = new_status else {
            continue;
        };
        let event = VoiceToTextCommandEvent::ConnectReceiver {
            api_key: api_key.clone(),
        };
        debug!("Now alive, sending event: {:?}", event);
        command_events.send(event);
    }
}
