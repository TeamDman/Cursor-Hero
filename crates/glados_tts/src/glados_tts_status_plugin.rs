use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::Instant;
use cursor_hero_glados_tts_types::prelude::*;

pub struct GladosTtsStatusPlugin;

impl Plugin for GladosTtsStatusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GladosTtsStatus>();
        app.add_systems(Update, handle_startup_event);
        app.add_systems(Update, periodic_ping);
        app.add_systems(Update, handle_pong);
    }
}

fn handle_startup_event(
    mut status_events: ParamSet<(
        EventReader<GladosTtsStatusEvent>,
        EventWriter<GladosTtsStatusEvent>,
    )>,
    mut glados_tts_status: ResMut<GladosTtsStatus>,
) {
    let starting = status_events
        .p0()
        .read()
        .any(|event| matches!(event, GladosTtsStatusEvent::Startup));
    if !starting {
        return;
    }
    status_events.p0().clear();

    *glados_tts_status = GladosTtsStatus::Starting {
        instant: Instant::now(),
        timeout: Duration::from_secs(5),
    };
    let event = GladosTtsStatusEvent::Changed {
        new_value: *glados_tts_status,
    };
    debug!("Sending event {:?}", event);
    status_events.p1().send(event);
}

fn periodic_ping(
    mut ping_events: EventWriter<GladosTtsPingEvent>,
    mut last_ping: Local<Option<Instant>>,
) {
    if let Some(instant) = *last_ping {
        if instant.elapsed().as_secs() > 5 {
            ping_events.send(GladosTtsPingEvent::Ping);
            *last_ping = Some(Instant::now());
        }
    } else {
        ping_events.send(GladosTtsPingEvent::Ping);
        *last_ping = Some(Instant::now());
    }
}

fn handle_pong(
    mut ping_events: EventReader<GladosTtsPingEvent>,
    mut status_events: EventWriter<GladosTtsStatusEvent>,
    mut gladosTts_status: ResMut<GladosTtsStatus>,
) {
    for event in ping_events.read() {
        match event {
            GladosTtsPingEvent::Pong { status } => {
                // identify the new state based on the pong
                let new_status = match (*gladosTts_status, *status) {
                    // if starting, only change to dead if the timeout has been exceeded
                    (GladosTtsStatus::Starting { instant, timeout }, status) => {
                        if status == GladosTtsStatus::Alive {
                            GladosTtsStatus::Alive
                        } else if instant.elapsed() > timeout {
                            GladosTtsStatus::Dead
                        } else {
                            GladosTtsStatus::Starting { instant, timeout }
                        }
                    }
                    // respect the new status if it's not starting
                    _ => *status,
                };

                if *gladosTts_status != new_status {
                    *gladosTts_status = new_status;
                    let event = GladosTtsStatusEvent::Changed {
                        new_value: new_status,
                    };
                    debug!("Sending event {:?}", event);
                    status_events.send(event);
                }
            }
            _ => {}
        }
    }
}
