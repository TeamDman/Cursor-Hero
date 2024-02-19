#![allow(dead_code)]

use bevy::prelude::*;

use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
// use crossbeam_channel::Sender;
use cursor_hero_winutils::win_events::message_loop;

pub struct HostEventPlugin;

impl Plugin for HostEventPlugin {
    #![allow(unused_variables)]
    fn build(&self, app: &mut App) {
        // not using this right now

        // app.add_systems(Startup, start_worker);
        // app.add_systems(Update, process_events);
    }
}

#[derive(Resource)]
struct EventBridge {
    receiver: Receiver<()>,
}

fn start_worker(mut commands: Commands) {
    info!("Starting worker thread");
    let (sender, receiver) = bounded::<()>(100);
    if let Err(e) = std::thread::Builder::new()
        .name("HostWatcher thread".to_string())
        // .spawn(move || {
        .spawn(|| {
            match cursor_hero_winutils::win_events::set_win_event_hook() {
                Ok(i) => {
                    info!("WinEventHook set: {}", i);
                    if let Err(()) = message_loop() {
                        error!("Message loop ended with an error.");
                    }
                }
                Err(_) => error!("Failed to set WinEventHook"),
            };
        })
    {
        error!("Failed to start worker thread: {:?}", e);
    };
    commands.insert_resource(EventBridge { receiver });
}

fn process_events(bridge: ResMut<EventBridge>) {
    for event in bridge.receiver.try_iter() {
        println!("Event: {:?}", event);
    }
}
