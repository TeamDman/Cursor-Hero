use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::ExitCondition;
use cursor_hero_memory_types::prelude::get_persist_file;
use cursor_hero_memory_types::prelude::MemoryConfig;
use cursor_hero_memory_types::prelude::Usage;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_worker::prelude::*;
use std::io::Write;
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                level: bevy::log::Level::DEBUG,
                filter: "
info,
wgpu_core=warn,
wgpu_hal=warn,
ui_snapshot_example=trace,
cursor_hero_worker=debug,
"
                .replace('\n', "")
                .trim()
                .into(),
            })
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..default()
            })
            .build(),
    );
    app.add_plugins(WorkerPlugin {
        config: WorkerConfig::<ThreadboundUISnapshotMessage, GameboundUISnapshotMessage, ()> {
            name: "ui_snapshot".to_string(),
            is_ui_automation_thread: true,
            handle_threadbound_message: handle_threadbound_message,
            ..default()
        },
    });
    app.add_systems(Update, trigger);
    app.add_systems(Update, receive);
    app.run();
}

#[derive(Debug, Reflect, Clone, Event)]
enum ThreadboundUISnapshotMessage {
    TakeSnapshot,
}
impl WorkerMessage for ThreadboundUISnapshotMessage {}

#[derive(Debug, Reflect, Clone, Event)]
enum GameboundUISnapshotMessage {
    Snapshot(UiSnapshot),
}
impl WorkerMessage for GameboundUISnapshotMessage {}

fn handle_threadbound_message(
    msg: &ThreadboundUISnapshotMessage,
    reply_tx: &Sender<GameboundUISnapshotMessage>,
    _state: &mut (),
) -> anyhow::Result<()> {
    let ThreadboundUISnapshotMessage::TakeSnapshot = msg;
    debug!("taking snapshot");
    let snapshot = take_snapshot()?;
    if let Err(e) = reply_tx.send(GameboundUISnapshotMessage::Snapshot(snapshot)) {
        error!("Failed to send snapshot: {:?}", e);
    }
    Ok(())
}

fn trigger(
    mut cooldown: Local<Option<Timer>>,
    time: Res<Time>,
    mut events: EventWriter<ThreadboundUISnapshotMessage>,
) {
    let should_tick = if let Some(cooldown) = cooldown.as_mut() {
        if cooldown.tick(time.delta()).just_finished() {
            cooldown.reset();
            true
        } else {
            false
        }
    } else {
        cooldown.replace(Timer::from_seconds(3.0, TimerMode::Repeating));
        true
    };
    if !should_tick {
        return;
    }
    events.send(ThreadboundUISnapshotMessage::TakeSnapshot);
}

fn receive(mut snapshot: EventReader<GameboundUISnapshotMessage>, memory_config: Res<MemoryConfig>) {
    for msg in snapshot.read() {
        match msg {
            GameboundUISnapshotMessage::Snapshot(snapshot) => {
                debug!("received snapshot, writing to file");
                match get_persist_file(
                    memory_config.as_ref(),
                    "ui_snapshot.txt",
                    Usage::Persist,
                ) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(snapshot.to_string().as_bytes()) {
                            debug!("Failed to write to file: {:?}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to open file: {:?}", e);
                    }
                }
            }
        }
    }
}
