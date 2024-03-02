use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
use cursor_hero_winutils::win_events::register_interest_in_mouse_with_os;
use cursor_hero_winutils::win_events::ProcMessage;

pub struct HostEventPlugin;

impl Plugin for HostEventPlugin {
    #![allow(unused_variables)]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_worker);
        app.add_systems(Update, process_events);
    }
}

#[derive(Resource)]
struct EventBridge {
    receiver: Receiver<()>,
}

fn start_worker(
    mut commands: Commands,
    window_query: Query<&RawHandleWrapper, With<PrimaryWindow>>,
) {
    info!("Starting worker thread");
    let (_sender, receiver) = bounded::<()>(100);

    let Ok(window_handle) = window_query.get_single() else {
        error!("Failed to get window handle");
        return;
    };
    let hwnd = match window_handle.window_handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => handle.hwnd as isize,
        _ => {
            error!("Unsupported window handle type");
            return;
        }
    };
    
    match register_interest_in_mouse_with_os(hwnd) {
        Ok(()) => info!("mouse interest registered with hwnd={:?}", hwnd),
        Err(e) => error!("Failed to register mouse interest: {:?}", e),
    };
    if let Err(e) = std::thread::Builder::new()
        .name("HostWatcher thread".to_string())
        .spawn(move || {
            // match register_interest_in_all_events_with_os_with_default_callback() {
            //     Ok(i) => info!("event interest registered hook={}", i),
            //     Err(e) => error!("Failed to register event interest: {:?}", e),
            // };
            
            match register_interest_in_mouse_with_os(hwnd) {
                Ok(()) => info!("mouse interest registered with hwnd={:?}", hwnd),
                Err(e) => error!("Failed to register mouse interest: {:?}", e),
            };

            debug!("Launching message loop");
            todo!();
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
