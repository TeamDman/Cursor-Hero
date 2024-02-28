use bevy::prelude::*;
use cursor_hero_ui_watcher_types::ui_watcher_types::AppUIElement;
use std::thread;

use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
pub struct UiWatcherPlugin;

impl Plugin for UiWatcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_worker_thread);
        app.add_systems(Update, handle_gamebound_messages);
        app.add_systems(Update, trigger_gather_app_info);
    }
}

#[derive(Debug)]
enum ThreadboundMessage {
    GatherAppInfo,
}

#[derive(Debug)]
enum GameboundMessage {
    AppInfo(String),
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
        ThreadboundMessage::GatherAppInfo => {
            let app_elements = cursor_hero_winutils::ui_automation::gather_toplevel_elements()?;
            let description = app_elements
                .into_iter()
                .map(|x| AppUIElement::from(x).to_string())
                .collect::<Vec<String>>()
                .join("\n");
            reply_tx.send(GameboundMessage::AppInfo(description))?;
        }
    }

    Ok(())
}

fn handle_gamebound_messages(bridge: Res<Bridge>) {
    while let Ok(msg) = bridge.receiver.try_recv() {
        match msg {
            GameboundMessage::AppInfo(app_info) => {
                info!("Received app info:\n```\n{}\n```", app_info);
            }
        }
    }
}

fn trigger_gather_app_info(
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
            if let Err(e) = bridge.sender.send(ThreadboundMessage::GatherAppInfo) {
                error!("Failed to send thread message: {:?}", e);
            }
            *cooldown = Some(Timer::from_seconds(5.0, TimerMode::Once));
        }
    }
}
