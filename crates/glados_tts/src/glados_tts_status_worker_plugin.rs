use bevy::prelude::*;
use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use cursor_hero_glados_tts_types::prelude::*;
use std::thread;

pub struct GladosTtsStatusWorkerPlugin;

impl Plugin for GladosTtsStatusWorkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_worker_thread);
        app.add_systems(Update, events_to_bridge);
        app.add_systems(Update, bridge_to_events);
    }
}

#[derive(Debug)]
enum GameboundMessage {
    Pong { status: GladosTtsStatus },
}

#[derive(Debug)]
enum ThreadboundMessage {
    Ping,
    Startup,
}

#[derive(Resource)]
struct Bridge {
    pub sender: Sender<ThreadboundMessage>,
    pub receiver: Receiver<GameboundMessage>,
}

fn create_worker_thread(mut commands: Commands) {
    let (game_tx, game_rx) = bounded::<_>(10);
    let (thread_tx, thread_rx) = bounded::<_>(10);
    commands.insert_resource(Bridge {
        sender: thread_tx,
        receiver: game_rx,
    });

    let game_tx_clone = game_tx.clone();
    thread::Builder::new()
        .name("GLaDOS TTS status thread".to_string())
        .spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let game_tx = game_tx_clone;
                loop {
                    let msg = match thread_rx.recv() {
                        Ok(msg) => msg,
                        Err(_) => {
                            error!("Threadbound channel failure, exiting");
                            break;
                        }
                    };
                    match msg {
                        ThreadboundMessage::Ping => {
                            debug!("Worker received ping request, pinging GladosTts API");
                            let status = match crate::glados_tts::get_status().await {
                                Ok(status) => status,
                                Err(e) => {
                                    error!("Failed to get status: {:?}", e);
                                    GladosTtsStatus::Unknown
                                }
                            };
                            if let Err(e) = game_tx.send(GameboundMessage::Pong { status }) {
                                error!("Gamebound channel failure, exiting: {}", e);
                                break;
                            }
                        }
                        ThreadboundMessage::Startup => {
                            debug!("Worker received startup request, starting GladosTts API");
                            if let Err(e) = crate::glados_tts::start() {
                                error!("Failed to start: {:?}", e);
                            };
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            });
        })
        .expect("Failed to spawn thread");
}

fn events_to_bridge(
    bridge: ResMut<Bridge>,
    mut ping_events: EventReader<GladosTtsPingEvent>,
    mut status_events: EventReader<GladosTtsStatusEvent>,
) {
    // Detect ping requests
    for event in ping_events.read() {
        let GladosTtsPingEvent::Ping = event else {
            continue;
        };
        let msg = ThreadboundMessage::Ping;
        debug!("Sending bridge message: {:?}", msg);
        if let Err(e) = bridge.sender.send(msg) {
            error!("Threadbound channel failure: {}", e);
        }
    }

    // Detect startup requests
    let starting = status_events
        .read()
        .any(|event| matches!(event, GladosTtsStatusEvent::Startup));
    if starting {
        status_events.clear();
        let msg = ThreadboundMessage::Startup;
        debug!("Sending bridge message: {:?}", msg);
        if let Err(e) = bridge.sender.send(msg) {
            error!("Threadbound channel failure: {}", e);
        }
    }
}

fn bridge_to_events(bridge: ResMut<Bridge>, mut events: EventWriter<GladosTtsPingEvent>) {
    for msg in bridge.receiver.try_iter() {
        match msg {
            GameboundMessage::Pong { status } => {
                let event = GladosTtsPingEvent::Pong { status };
                debug!("Received bridge response, sending game event {:?}", event);
                events.send(event);
            }
        }
    }
}
