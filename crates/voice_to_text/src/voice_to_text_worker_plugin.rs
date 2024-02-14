use bevy::prelude::*;
use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use cursor_hero_secret_types::secrets_types::SecretString;
use cursor_hero_voice_to_text_types::prelude::*;
use std::thread;
use std::time::Duration;
use std::time::Instant;

pub struct VoiceToTextWorkerPlugin;

impl Plugin for VoiceToTextWorkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_worker_thread);
        app.add_systems(Update, events_to_bridge);
        app.add_systems(Update, bridge_to_events);
    }
}

#[derive(Debug)]
pub(crate) enum GameboundMessage {
    Pong { status: VoiceToTextStatus },
    Starting { api_key: SecretString },
    TranscriptionReceived { transcription: String },
}

#[derive(Debug)]
enum ThreadboundMessage {
    Ping,
    Startup,
    SetListening {
        listening: bool,
        api_key: SecretString,
    },
    ConnectReceiver {
        api_key: SecretString,
    },
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
        .name("Voice2Text status thread".to_string())
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
                            debug!("Worker received ping request, pinging VoiceToText API");
                            let status = match crate::voice_to_text::get_status().await {
                                Ok(status) => status,
                                Err(e) => {
                                    error!("Failed to get status: {:?}", e);
                                    VoiceToTextStatus::Unknown
                                }
                            };
                            if let Err(e) = game_tx.send(GameboundMessage::Pong { status }) {
                                error!("Gamebound channel failure, exiting: {}", e);
                                break;
                            }
                        }
                        ThreadboundMessage::Startup => {
                            debug!("Worker received startup request, starting VoiceToText API");
                            match crate::voice_to_text::start() {
                                Ok(api_key) => {
                                    debug!("VoiceToText API started successfully");
                                    if let Err(e) =
                                        game_tx.send(GameboundMessage::Starting { api_key })
                                    {
                                        error!("Gamebound channel failure, exiting: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to start: {:?}", e);
                                }
                            };
                        }
                        ThreadboundMessage::SetListening { listening, api_key } => {
                            debug!("Worker received set listening request: {}", listening);
                            match crate::voice_to_text::set_listening(listening, api_key).await {
                                Ok(()) => {
                                    info!(
                                        "VoiceToText API set listening={} successfully",
                                        listening
                                    );
                                }
                                Err(e) => {
                                    error!("Failed to set listening: {:?}", e);
                                }
                            }
                        }
                        ThreadboundMessage::ConnectReceiver { api_key } => {
                            debug!("Worker received connect receiver request");
                            match crate::voice_to_text::connect_receiver(game_tx.clone(), api_key)
                                .await
                            {
                                Ok(()) => {
                                    debug!("VoiceToText API connected receiver successfully");
                                }
                                Err(e) => {
                                    error!("Failed to connect receiver: {:?}", e);
                                }
                            }
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
    mut ping_events: EventReader<VoiceToTextPingEvent>,
    mut command_events: EventReader<VoiceToTextCommandEvent>,
) {
    // Detect ping requests
    for event in ping_events.read() {
        let VoiceToTextPingEvent::Ping = event else {
            continue;
        };
        let msg = ThreadboundMessage::Ping;
        debug!("Sending bridge message: {:?}", msg);
        if let Err(e) = bridge.sender.send(msg) {
            error!("Threadbound channel failure: {}", e);
        }
    }

    for event in command_events.read() {
        let msg = match event {
            VoiceToTextCommandEvent::Startup => ThreadboundMessage::Startup,
            VoiceToTextCommandEvent::SetListening { listening, api_key } => {
                ThreadboundMessage::SetListening {
                    listening: *listening,
                    api_key: api_key.clone(),
                }
            }
            VoiceToTextCommandEvent::ConnectReceiver { api_key } => {
                ThreadboundMessage::ConnectReceiver {
                    api_key: api_key.clone(),
                }
            }
        };
        debug!("Sending bridge message: {:?}", msg);
        if let Err(e) = bridge.sender.send(msg) {
            error!("Threadbound channel failure: {}", e);
        }
    }
}

fn bridge_to_events(
    bridge: ResMut<Bridge>,
    mut ping_events: EventWriter<VoiceToTextPingEvent>,
    mut status_events: EventWriter<VoiceToTextStatusEvent>,
    mut transcription_events: EventWriter<VoiceToTextTranscriptionEvent>,
    mut current_status: ResMut<VoiceToTextStatus>,
) {
    for msg in bridge.receiver.try_iter() {
        match msg {
            GameboundMessage::Pong { status } => {
                let event = VoiceToTextPingEvent::Pong { status };
                debug!("Received bridge response, sending game event {:?}", event);
                ping_events.send(event);
            }
            GameboundMessage::Starting { api_key } => {
                let new_status = VoiceToTextStatus::Starting {
                    instant: Instant::now(),
                    timeout: Duration::from_secs(60),
                    api_key: api_key.clone(),
                };
                let event = VoiceToTextStatusEvent::Changed {
                    old_status: current_status.clone(),
                    new_status: new_status.clone(),
                };
                debug!("Received bridge response, sending game event {:?}", event);
                status_events.send(event);

                *current_status = new_status;
            }
            GameboundMessage::TranscriptionReceived { transcription } => {
                let event = VoiceToTextTranscriptionEvent::Received { transcription };
                debug!("Received bridge response, sending game event {:?}", event);
                transcription_events.send(event);
            }
        }
    }
}
