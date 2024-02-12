use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::Instant;
use cursor_hero_voice_to_text_types::prelude::*;

pub struct VoiceToTextStatusPlugin;

impl Plugin for VoiceToTextStatusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoiceToTextStatus>();
        app.add_systems(Update, handle_startup_event);
        app.add_systems(Update, periodic_ping);
        app.add_systems(Update, handle_pong);
    }
}

fn handle_startup_event(
    mut status_events: ParamSet<(
        EventReader<VoiceToTextStatusEvent>,
        EventWriter<VoiceToTextStatusEvent>,
    )>,
    mut voice_to_text_status: ResMut<VoiceToTextStatus>,
) {
    let starting = status_events
        .p0()
        .read()
        .any(|event| matches!(event, VoiceToTextStatusEvent::Startup));
    if !starting {
        return;
    }
    status_events.p0().clear();

    *voice_to_text_status = VoiceToTextStatus::Starting {
        instant: Instant::now(),
        timeout: Duration::from_secs(5),
    };
    let event = VoiceToTextStatusEvent::Changed {
        new_value: *voice_to_text_status,
    };
    debug!("Sending event {:?}", event);
    status_events.p1().send(event);
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
    mut voice_to_text_status: ResMut<VoiceToTextStatus>,
) {
    for event in ping_events.read() {
        let VoiceToTextPingEvent::Pong { status } = event else {
            continue;
        };
        // identify the new state based on the pong
        let new_status = match (*voice_to_text_status, *status) {
            // if starting, only change to dead if the timeout has been exceeded
            (VoiceToTextStatus::Starting { instant, timeout }, status) => {
                if status == VoiceToTextStatus::Alive {
                    VoiceToTextStatus::Alive
                } else if instant.elapsed() > timeout {
                    VoiceToTextStatus::Dead
                } else {
                    VoiceToTextStatus::Starting { instant, timeout }
                }
            }
            // respect the new status if it's not starting
            _ => *status,
        };

        if *voice_to_text_status != new_status {
            *voice_to_text_status = new_status;
            let event = VoiceToTextStatusEvent::Changed {
                new_value: new_status,
            };
            debug!("Sending event {:?}", event);
            status_events.send(event);
        }
    }
}
