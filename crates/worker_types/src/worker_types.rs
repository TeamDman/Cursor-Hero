use anyhow::Error;
use anyhow::Result;
use bevy::prelude::*;
pub use crossbeam_channel::Receiver;
pub use crossbeam_channel::Sender;
use std::marker::PhantomData;

#[derive(Resource)]
pub struct Bridge<T, G>
where
    T: WorkerMessage,
    G: WorkerMessage,
{
    pub sender: Sender<T>,
    pub receiver: Receiver<G>,
}

pub trait WorkerMessage: std::fmt::Debug + Event + Send + Sync + Clone + 'static {}

pub trait WorkerState: 'static + Sized
//  + Send + Sync + Clone
{
    fn try_default() -> Result<Self>;
}
impl WorkerState for () {
    fn try_default() -> Result<Self> {
        Ok(())
    }
}

pub type ThreadboundMessageHandler<T, G, S> =
    fn(msg: &T, reply_tx: &Sender<G>, state: &mut S) -> Result<()>;

pub type ThreadboundMessageErrorHandler<T, G, S> =
    fn(msg: &T, reply_tx: &Sender<G>, state: &mut S, error: &Error) -> Result<()>;

pub type ThreadboundMessageReceiver<T, S> = fn(thread_rx: &Receiver<T>, state: &mut S) -> Result<T>;


struct TGSHolder<T,G,S> {
    _phantom_t: PhantomData<T>,
    _phantom_g: PhantomData<G>,
    _phantom_s: PhantomData<S>,
}
unsafe impl <T, G, S> Send for TGSHolder<T, G, S> {}
unsafe impl <T, G, S> Sync for TGSHolder<T, G, S> {}
impl <T, G, S> Clone for TGSHolder<T, G, S> {
    fn clone(&self) -> Self {
        TGSHolder {
            _phantom_t: PhantomData,
            _phantom_g: PhantomData,
            _phantom_s: PhantomData,
        }
    }
}

#[derive(Resource, Reflect)]
pub struct WorkerConfig<T, G, S> {
    pub name: String,
    pub sleep_duration: std::time::Duration,
    pub is_ui_automation_thread: bool,
    pub threadbound_message_receiver: ThreadboundMessageReceiver<T, S>,
    pub handle_threadbound_message: ThreadboundMessageHandler<T, G, S>,
    pub handle_threadbound_message_error_handler: ThreadboundMessageErrorHandler<T, G, S>,
    pub gamebound_channel_capacity: usize,
    pub threadbound_channel_capacity: usize,
    pub tgs_holder: TGSHolder<T, G, S>,
}
impl<T: WorkerMessage, G: WorkerMessage, S: WorkerState> Default for WorkerConfig<T, G, S> {
    fn default() -> Self {
        WorkerConfig {
            name: "Unknown Worker".to_string(),
            is_ui_automation_thread: false,
            sleep_duration: std::time::Duration::from_millis(100),
            handle_threadbound_message: |_, _, _| Ok(()),
            handle_threadbound_message_error_handler: |_, _, _, _| Ok(()),
            threadbound_message_receiver: |thread_rx, _state| {
                thread_rx
                    .recv()
                    .map_err(|e| Error::from(e).context("receiving threadbound message"))
            },
            gamebound_channel_capacity: 10,
            threadbound_channel_capacity: 10,
            tgs_holder: TGSHolder {
                _phantom_t: PhantomData,
                _phantom_g: PhantomData,
                _phantom_s: PhantomData,
            },
        }
    }
}
impl <T,G,S> Clone for WorkerConfig<T,G,S> {
    fn clone(&self) -> Self {
        WorkerConfig {
            name: self.name.clone(),
            sleep_duration: self.sleep_duration,
            is_ui_automation_thread: self.is_ui_automation_thread,
            threadbound_message_receiver: self.threadbound_message_receiver.clone(),
            handle_threadbound_message: self.handle_threadbound_message.clone(),
            handle_threadbound_message_error_handler: self.handle_threadbound_message_error_handler.clone(),
            gamebound_channel_capacity: self.gamebound_channel_capacity,
            threadbound_channel_capacity: self.threadbound_channel_capacity,
            tgs_holder: self.tgs_holder.clone(),
        }
    }
}
