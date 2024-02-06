use bevy::prelude::*;
use std::thread;

use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use cursor_hero_inference_types::prelude::*;
pub struct OllamaPlugin;

impl Plugin for OllamaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_worker_thread);
        app.add_systems(Update, bridge_generate_requests);
        app.add_systems(Update, bridge_generate_responses);
    }
}

#[derive(Debug)]
enum GameboundMessage {
    Response {
        session_id: Entity,
        prompt: String,
        response: String,
    },
}

#[derive(Debug)]
enum ThreadboundMessage {
    Generate { session_id: Entity, prompt: String },
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
        .name("Ollama thread".to_string())
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
                        ThreadboundMessage::Generate { session_id, prompt } => {
                            debug!("Worker received generate request for session {:?}, generating response", session_id);
                            let data = match crate::ollama::generate(&prompt).await {
                                Ok(data) => data,
                                Err(e) => {
                                    error!("Failed to generate: {}", e);
                                    continue;
                                }
                            };
                            if let Err(e) = game_tx.send(GameboundMessage::Response {
                                session_id,
                                prompt: prompt.clone(),
                                response: data,
                            }) {
                                error!("Gamebound channel failure, exiting: {}", e);
                                break;
                            }
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            });
        })
        .expect("Failed to spawn thread");
}

fn bridge_generate_requests(bridge: ResMut<Bridge>, mut events: EventReader<InferenceEvent>) {
    for event in events.read() {
        if let InferenceEvent::Request { session_id, prompt } = event {
            debug!(
                "Received generate request for session {:?}, sending over bridge to worker thread",
                session_id
            );
            if let Err(e) = bridge.sender.send(ThreadboundMessage::Generate {
                session_id: *session_id,
                prompt: prompt.clone(),
            }) {
                error!("Threadbound channel failure: {}", e);
            }
        }
    }
}

fn bridge_generate_responses(bridge: ResMut<Bridge>, mut events: EventWriter<InferenceEvent>) {
    for msg in bridge.receiver.try_iter() {
        match msg {
            GameboundMessage::Response {
                session_id,
                prompt,
                response,
            } => {
                let event = InferenceEvent::Response {
                    session_id,
                    response,
                    prompt,
                };
                debug!("Received bridge response, sending game event {:?}", event);
                events.send(event);
            }
        }
    }
}
