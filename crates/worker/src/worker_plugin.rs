use bevy::prelude::*;
use crossbeam_channel::bounded;
use cursor_hero_worker_types::prelude::*;
use std::thread;

use windows::Win32::System::Com::CoInitializeEx;
use windows::Win32::System::Com::COINIT_MULTITHREADED;
pub struct WorkerPlugin<T, G, S>
where
    T: WorkerMessage,
    G: WorkerMessage,
    S: WorkerState,
{
    pub config: WorkerConfig<T, G, S>,
}

impl<T, G, S> Plugin for WorkerPlugin<T, G, S>
where
    T: WorkerMessage,
    G: WorkerMessage,
    S: WorkerState,
{
    fn build(&self, app: &mut App) {
        // TODO: conditionally register if T or G support it
        // app.register_type::<T>();
        // app.register_type::<G>();
        app.add_event::<T>();
        app.add_event::<G>();
        app.insert_resource(self.config.clone());
        app.add_systems(Startup, create_worker_thread::<T, G, S>);
        app.add_systems(Update, bridge_requests::<T, G, S>);
        app.add_systems(Update, bridge_responses::<T, G, S>);
    }
}

fn create_worker_thread<T: WorkerMessage, G: WorkerMessage, S: WorkerState>(
    config: Res<WorkerConfig<T, G, S>>,
    mut commands: Commands,
) {
    let (game_tx, game_rx) = bounded::<G>(config.gamebound_channel_capacity);
    let (thread_tx, thread_rx) = bounded::<T>(config.threadbound_channel_capacity);

    commands.insert_resource(Bridge {
        sender: thread_tx,
        receiver: game_rx,
    });

    let name = config.name.clone();
    let handler = config.handle_threadbound_message;
    let handler_error_handler = config.handle_threadbound_message_error_handler;
    let sleep_duration = config.sleep_duration;
    let is_ui_automation_thread = config.is_ui_automation_thread;
    let receiver = config.threadbound_message_receiver;
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

        let Ok(mut state) = S::try_default() else {
            error!("[{}] Failed to initialize state", name);
            return;
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                let msg = match (receiver)(&thread_rx, &mut state) {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!("[{}] Threadbound channel receiver failure: {:?}, quitting loop", name, e);
                        break;
                    }
                };
                if let Err(e) = (handler)(&msg, &game_tx, &mut state) {
                    // TODO: leave logging the error to the handler
                    error!(
                        "[{}] Failed to process thread message {:?}, got error {:?}",
                        name, msg, e
                    );
                    if let Err(ee) = (handler_error_handler)(&msg, &game_tx, &mut state, &e) {
                        error!(
                            "[{}] BAD NEWS! Failed while processing error handler for message {:?} that produced error {:?}, got new error {:?}",
                            name, msg, e, ee
                        );
                    }
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

fn bridge_requests<T: WorkerMessage, G: WorkerMessage, S: WorkerState>(
    config: Res<WorkerConfig<T, G, S>>,
    bridge: ResMut<Bridge<T, G>>,
    mut events: EventReader<T>,
) {
    for event in events.read() {
        trace!("[{}] Bevy => Thread: {:?}", config.name, event);
        if let Err(e) = bridge.sender.try_send(event.clone()) {
            match e {
                crossbeam_channel::TrySendError::Full(_) => {
                    error!("[{}] Threadbound channel is full, dropping message: {:?}", config.name, event);
                }
                crossbeam_channel::TrySendError::Disconnected(_) => {
                    error!("[{}] Threadbound channel is disconnected, dropping message: {:?}", config.name, event);
                }
            }
        }
    }
}

fn bridge_responses<T: WorkerMessage, G: WorkerMessage, S: WorkerState>(
    config: Res<WorkerConfig<T, G, S>>,
    bridge: ResMut<Bridge<T, G>>,
    mut events: EventWriter<G>,
) {
    for msg in bridge.receiver.try_iter() {
        trace!("[{}] Thread => Bevy: {:?}", config.name, msg);
        events.send(msg);
    }
}
