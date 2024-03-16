use bevy::prelude::*;
use cursor_hero_character_types::prelude::MainCharacter;
use cursor_hero_environment_types::environment_types::EnvironmentTag;
use cursor_hero_memory_types::prelude::get_persist_file;
use cursor_hero_memory_types::prelude::Usage;
use cursor_hero_observation_types::observation_types::SomethingObservableHappenedEvent;
use cursor_hero_ui_automation::prelude::take_snapshot;
use cursor_hero_ui_automation::prelude::UISnapshot;
use std::io::Write;
use std::thread;

use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
pub struct UiWatcherPlugin;

impl Plugin for UiWatcherPlugin {
    fn build(&self, app: &mut App) {
        return;
        app.add_systems(Startup, spawn_worker_thread);
        app.add_systems(Update, handle_gamebound_messages);
        app.add_systems(Update, trigger_gather_info);
    }
}

#[derive(Debug)]
enum ThreadboundMessage {
    TakeSnapshot,
}

#[derive(Debug)]
enum GameboundMessage {
    Snapshot(UISnapshot),
}

#[derive(Resource)]
struct Bridge {
    pub sender: Sender<ThreadboundMessage>,
    pub receiver: Receiver<GameboundMessage>,
}

fn spawn_worker_thread(mut commands: Commands) {
    let (tx, rx) = bounded::<_>(10);
    let (reply_tx, reply_rx) = bounded::<_>(10); // New channel for replies

    commands.insert_resource(Bridge {
        sender: tx,
        receiver: reply_rx,
    });
    thread::spawn(move || loop {
        let action = match rx.recv() {
            Ok(action) => action,
            Err(e) => {
                error!("Failed to receive thread message, exiting: {:?}", e);
                break;
            }
        };
        if let Err(e) = handle_threadbound_messages(action, &reply_tx) {
            error!("Failed to process thread message: {:?}", e);
        }
    });
}

fn handle_threadbound_messages(
    action: ThreadboundMessage,
    reply_tx: &Sender<GameboundMessage>,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ThreadboundMessage::TakeSnapshot => {
            let snapshot = take_snapshot()?;
            let msg = GameboundMessage::Snapshot(snapshot);
            // println!("Sending {:?}", msg);
            reply_tx.send(msg)?;
        }
    }

    Ok(())
}

fn handle_gamebound_messages(
    bridge: Res<Bridge>,
    mut observation_events: EventWriter<SomethingObservableHappenedEvent>,
    character_query: Query<&EnvironmentTag, With<MainCharacter>>,
) {
    let Ok(character) = character_query.get_single() else {
        warn!("Expected single main character, failed");
        while let Ok(_msg) = bridge.receiver.try_recv() { // drain the channel
        }
        return;
    };
    let character_environment = character;
    while let Ok(msg) = bridge.receiver.try_recv() {
        let (msg_kind, GameboundMessage::Snapshot(snapshot)) = ("Snapshot", msg);
        debug!("Received message {}:\n{}", msg_kind, snapshot);

        observation_events.send(SomethingObservableHappenedEvent::UISnapshot {
            snapshot: snapshot.clone(),
            environment_id: Some(character_environment.environment_id),
        });

        match get_persist_file(file!(), "results.txt", Usage::Persist) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(snapshot.to_string().as_bytes()) {
                    error!("Failed to write to file: {:?}", e);
                }
            }
            Err(e) => {
                error!("Failed to open file: {:?}", e);
            }
        }
    }
}

fn trigger_gather_info(
    bridge: ResMut<Bridge>,
    mut cooldown: Local<Option<Timer>>,
    time: Res<Time>,
) {
    match *cooldown {
        Some(ref mut timer) => {
            if timer.tick(time.delta()).just_finished() {
                *cooldown = None;
            }
        }
        None => {
            debug!("Triggering gather info");
            // if let Err(e) = bridge.sender.send(ThreadboundMessage::GatherFocusInfo) {
            //     error!("Failed to send thread message: {:?}", e);
            // }
            if let Err(e) = bridge.sender.send(ThreadboundMessage::TakeSnapshot) {
                error!("Failed to send thread message: {:?}", e);
            }
            *cooldown = Some(Timer::from_seconds(5.0, TimerMode::Once));
        }
    }
}
