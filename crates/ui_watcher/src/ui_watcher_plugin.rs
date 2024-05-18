use bevy::prelude::*;
use cursor_hero_character_types::prelude::MainCharacter;
use cursor_hero_environment_types::environment_types::TrackedEnvironment;
use cursor_hero_memory_types::prelude::get_persist_file;
use cursor_hero_memory_types::prelude::MemoryConfig;
use cursor_hero_memory_types::prelude::Usage;
use cursor_hero_observation_types::observation_types::SomethingObservableHappenedEvent;
use cursor_hero_ui_automation::prelude::take_snapshot;
use cursor_hero_ui_watcher_types::ui_watcher_types::GameboundUIWatcherMessage;
use cursor_hero_ui_watcher_types::ui_watcher_types::ThreadboundUIWatcherMessage;
use cursor_hero_worker::prelude::anyhow::Result;
use cursor_hero_worker::prelude::Sender;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerPlugin;
use std::io::Write;

pub struct UiWatcherPlugin;

impl Plugin for UiWatcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorkerPlugin {
            config: WorkerConfig::<ThreadboundUIWatcherMessage, GameboundUIWatcherMessage, (), _,_,_> {
                name: "ui watcher".to_string(),
                handle_threadbound_message,
                ..default()
            },
        });
        app.add_systems(Update, handle_gamebound_messages);
        app.add_systems(Update, trigger_gather_info);
    }
}

fn handle_threadbound_message(
    msg: &ThreadboundUIWatcherMessage,
    reply_tx: &Sender<GameboundUIWatcherMessage>,
    _state: &mut (),
) -> Result<()> {
    match msg {
        ThreadboundUIWatcherMessage::TakeSnapshot => {
            let snapshot = take_snapshot()?;
            let msg = GameboundUIWatcherMessage::Snapshot(snapshot);
            // println!("Sending {:?}", msg);
            reply_tx.send(msg)?;
        }
    }

    Ok(())
}

fn handle_gamebound_messages(
    memory_config: Res<MemoryConfig>,
    mut gamebound_events: EventReader<GameboundUIWatcherMessage>,
    mut observation_events: EventWriter<SomethingObservableHappenedEvent>,
    character_query: Query<&TrackedEnvironment, With<MainCharacter>>,
) {
    if gamebound_events.is_empty() {
        return;
    }
    let environment_id = character_query.get_single().ok().map(|c| c.environment_id);
    for msg in gamebound_events.read() {
        let (msg_kind, GameboundUIWatcherMessage::Snapshot(snapshot)) = ("Snapshot", msg);
        debug!("Received message {}", msg_kind);

        observation_events.send(SomethingObservableHappenedEvent::UISnapshot {
            snapshot: snapshot.clone(),
            environment_id,
        });

        match get_persist_file(memory_config.as_ref(), "results.txt", Usage::Persist) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(format!("{:#?}", snapshot).as_bytes()) {
                    error!("Failed to write to file: {:?}", e);
                } else {
                    info!("Wrote snapshot to file {:?}", file);
                }
            }
            Err(e) => {
                error!("Failed to open file: {:?}", e);
            }
        }
    }
}

fn trigger_gather_info(
    mut events: EventWriter<ThreadboundUIWatcherMessage>,
    mut cooldown: Local<Option<Timer>>,
    time: Res<Time>,
) {
    // handle cooldown
    let Some(cooldown) = cooldown.as_mut() else {
        cooldown.replace(Timer::from_seconds(1.0, TimerMode::Repeating));
        return;
    };
    if !cooldown.tick(time.delta()).just_finished() {
        return;
    }

    // send event to worker
    debug!("Triggering gather info");
    events.send(ThreadboundUIWatcherMessage::TakeSnapshot);
}
