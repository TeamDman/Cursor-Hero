use anyhow::Error;
use anyhow::Result;
use bevy::prelude::*;
pub use crossbeam_channel::Receiver;
pub use crossbeam_channel::Sender;
use std::marker::PhantomData;

#[derive(Resource)]
pub struct Bridge<T, G>
where
    T: Message,
    G: Message,
{
    pub sender: Sender<T>,
    pub receiver: Receiver<G>,
}

pub trait Message:
    std::fmt::Debug
    // + GetTypeRegistration
    + Event
    + Send
    + Sync
    + Clone
    // + Reflect
    // + TypePath
    // + FromReflect
    + 'static
{
}

pub type ThreadboundMessageHandler<T, G> = fn(msg: &T, reply_tx: &Sender<G>) -> Result<()>;

pub type ThreadboundMessageErrorHandler<T, G> =
    fn(msg: &T, reply_tx: &Sender<G>, error: &Error) -> Result<()>;

pub type ThreadboundMessageReceiver<T> = fn(thread_rx: &Receiver<T>) -> Result<T>;

#[derive(Resource, Reflect, Clone)]
pub struct WorkerConfig<T, G> {
    pub name: String,
    pub sleep_duration: std::time::Duration,
    pub is_ui_automation_thread: bool,
    pub threadbound_message_receiver: ThreadboundMessageReceiver<T>,
    pub handle_threadbound_message: ThreadboundMessageHandler<T, G>,
    pub handle_threadbound_message_error_handler: ThreadboundMessageErrorHandler<T, G>,
    pub _phantom_t: PhantomData<T>,
    pub _phantom_g: PhantomData<G>,
}
impl<T: Message, G: Message> Default for WorkerConfig<T, G> {
    fn default() -> Self {
        WorkerConfig {
            name: "Unknown Worker".to_string(),
            is_ui_automation_thread: false,
            sleep_duration: std::time::Duration::from_millis(100),
            handle_threadbound_message: |_, _| Ok(()),
            handle_threadbound_message_error_handler: |_, _, _| Ok(()),
            threadbound_message_receiver: |thread_rx| {
                thread_rx
                    .recv()
                    .map_err(|e| Error::from(e).context("receiving threadbound message"))
            },
            _phantom_t: PhantomData,
            _phantom_g: PhantomData,
        }
    }
}
