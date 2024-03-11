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
        app.register_type::<T>();
        app.register_type::<G>();
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
    thread::Builder::new()
        .name(name.clone())
        .spawn(move || {
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

            // {
            //     // Enable bevy logging in the thread
            //     // source: https://discord.com/channels/691052431525675048/1070649194739679262/1070987678813782046

            //     // We need to be able to inject this layer as our formatting layer
            //     let fmt_layer = tracing_subscriber::fmt::Layer::default().with_thread_ids(true);

            //     // The rest of this we just copy-paste directly from the LogPlugin
            //     // Note this does not include some feature-gated logic
            //     let default_filter = "warn,log_threadid=warn,bevy_ecs=error".to_string();
            //     LogTracer::init().unwrap();
            //     let filter_layer = EnvFilter::try_from_default_env()
            //         .or_else(|_| EnvFilter::try_new(&default_filter))
            //         .unwrap();
            //     let subscriber = Registry::default().with(filter_layer);
            //     let subscriber = subscriber.with(fmt_layer);
            //     bevy::utils::tracing::subscriber::set_global_default(subscriber).unwrap();
            // }

            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                loop {
                    let msg = match thread_rx.recv() {
                        Ok(msg) => msg,
                        Err(e) => {
                            error!("[{}] Threadbound channel recv failure {:?}, exiting: ", name, e);
                            break;
                        }
                    };
                    if let Err(e) = (handler)(&msg, &game_tx) {
                        error!(
                            "[{}] Failed to process thread message {:?}, got error {:?}",
                            name, msg, e
                        );
                    }
                    std::thread::sleep(sleep_duration);
                }
            });
        })
        .expect(format!("[{}] Failed to spawn thread", config.name).as_str());
    info!("[{}] Thread created", config.name);
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
