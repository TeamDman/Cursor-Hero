use bevy::prelude::*;
use cursor_hero_ui_automation::prelude::gather_apps;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
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
            let description = gather_apps()?
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            let msg = GameboundMessage::AppInfo(description);
            println!("Sending {:?}", msg);
            reply_tx.send(msg)?;
        }
    }

    Ok(())
}

fn get_persist_file(current_path: &str, file_name: &str) -> Result<std::fs::File, std::io::Error> {
    let mut file_path = PathBuf::new();

    #[cfg(debug_assertions)]
    {
        let dir = Path::new(current_path).parent().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Parent not found",
        ))?;
        file_path.push(dir);
    }

    file_path.push(file_name);
    let handle = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;
    Ok(handle)
}

fn handle_gamebound_messages(bridge: Res<Bridge>) {
    while let Ok(msg) = bridge.receiver.try_recv() {
        match msg {
            GameboundMessage::AppInfo(app_info) => {
                debug!("Received app info, length: {}", app_info.len());
                match get_persist_file(file!(), "results.txt") {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(app_info.as_bytes()) {
                            error!("Failed to write to file: {:?}", e);
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
            debug!("Triggering gather app info");
            if let Err(e) = bridge.sender.send(ThreadboundMessage::GatherAppInfo) {
                error!("Failed to send thread message: {:?}", e);
            }
            *cooldown = Some(Timer::from_seconds(5.0, TimerMode::Once));
        }
    }
}

#[cfg(test)]
mod tests {
    use cursor_hero_ui_automation::prelude::gather_toplevel_elements;
    use cursor_hero_ui_automation::prelude::AppUIElement;

    #[test]
    fn test_gather_app_info() -> Result<(), Box<dyn std::error::Error>> {
        println!("Gathering app info");
        let app_elements = gather_toplevel_elements()?;
        println!("Gathered {} elements, processing...", app_elements.len());
        let description = app_elements
            .into_iter()
            .map(|x| AppUIElement::from(x).to_string())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", description);
        Ok(())
    }
}
