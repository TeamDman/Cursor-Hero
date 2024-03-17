use bevy::prelude::*;
use crossbeam_channel::bounded;
use cursor_hero_worker_types::prelude::*;
use std::thread;

use windows::Win32::System::Com::CoInitializeEx;
use windows::Win32::System::Com::COINIT_MULTITHREADED;
pub struct WorkerPlugin<T, G>
where
    T: Message,
    G: Message,
{
    pub config: WorkerConfig<T, G>,
}

impl<T, G> Plugin for WorkerPlugin<T, G>
where
    T: Message,
    G: Message,
{
    fn build(&self, app: &mut App) {
        // TODO: conditionally register if T or G support it
        // app.register_type::<T>();
        // app.register_type::<G>();
        app.add_event::<T>();
        app.add_event::<G>();
        app.insert_resource(self.config.clone());
        app.add_systems(Startup, create_worker_thread::<T, G>);
        app.add_systems(Update, bridge_requests::<T, G>);
        app.add_systems(Update, bridge_responses::<T, G>);
    }
}

fn create_worker_thread<T: Message, G: Message>(
    config: Res<WorkerConfig<T, G>>,
    mut commands: Commands,
) {
    let (game_tx, game_rx) = bounded::<G>(10);
    let (thread_tx, thread_rx) = bounded::<T>(10);

    commands.insert_resource(Bridge {
        sender: thread_tx,
        receiver: game_rx,
    });

    let name = config.name.clone();
    let handler = config.handle_threadbound_message;
    let sleep_duration = config.sleep_duration;
    let is_ui_automation_thread = config.is_ui_automation_thread;
    if let Err(e) = thread::Builder::new().name(name.clone()).spawn(move || {
        if is_ui_automation_thread {
            unsafe {
                // Initialize COM in MTA mode
                // https://learn.microsoft.com/en-us/dotnet/framework/ui-automation/ui-automation-threading-issues
                // https://learn.microsoft.com/en-us/windows/win32/com/multithreaded-apartments
                if let Err(e) = CoInitializeEx(None, COINIT_MULTITHREADED) {
                    error!("[{}] Failed to initialize COM: {:?}", name, e);
                }
                debug!("[{}] COM initialized in MTA mode.", name);
            }
        }

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                let msg = match thread_rx.recv() {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!(
                            "[{}] Threadbound channel recv failure {:?}, exiting: ",
                            name, e
                        );
                        break;
                    }
                };
                if let Err(e) = (handler)(&msg, &game_tx) {
                    // eprintln cause bevy logging isn't guaranteed to display
                    eprintln!(
                        "[{}] Failed to process thread message {:?}, got error {:?}",
                        name, msg, e
                    );
                    error!(
                        "[{}] Failed to process thread message {:?}, got error {:?}",
                        name, msg, e
                    );
                }
                std::thread::sleep(sleep_duration);
            }
        });
    }) {
        error!("[{}] Failed to spawn thread: {:?}", config.name, e);
    } else {
        info!("[{}] Thread created", config.name);
    }
}

fn bridge_requests<T: Message, G: Message>(
    config: Res<WorkerConfig<T, G>>,
    bridge: ResMut<Bridge<T, G>>,
    mut events: EventReader<T>,
) {
    for event in events.read() {
        trace!("[{}] Bevy => Thread: {:?}", config.name, event);
        if let Err(e) = bridge.sender.send(event.clone()) {
            error!("[{}] Threadbound channel failure: {:?}", config.name, e);
        }
    }
}

fn bridge_responses<T: Message, G: Message>(
    config: Res<WorkerConfig<T, G>>,
    bridge: ResMut<Bridge<T, G>>,
    mut events: EventWriter<G>,
) {
    for msg in bridge.receiver.try_iter() {
        trace!("[{}] Thread => Bevy: {:?}", config.name, msg);
        events.send(msg);
    }
}
