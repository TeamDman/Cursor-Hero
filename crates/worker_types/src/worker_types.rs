// use anyhow::Error;
// use anyhow::Result;
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
impl<T> WorkerMessage for T where T: std::fmt::Debug + Event + Send + Sync + Clone + 'static {}

pub trait WorkerError: std::fmt::Debug + 'static {}
impl<T> WorkerError for T where T: std::fmt::Debug + 'static {}

pub trait WorkerState: 'static + Sized //  + Send + Sync + Clone
{
    type Error;
    fn try_default() -> Result<Self, Self::Error>;
}
impl WorkerState for () {
    type Error = ();
    fn try_default() -> Result<Self, Self::Error> {
        Ok(())
    }
}

pub type ThreadboundMessageHandler<T, G, S, E> =
    fn(msg: &T, reply_tx: &Sender<G>, state: &mut S) -> Result<(), E>;

pub type ThreadboundMessageErrorHandler<T, G, S, ErrorFromMsgHandling, ErrorFromErrorHandling> =
    fn(
        msg: &T,
        reply_tx: &Sender<G>,
        state: &mut S,
        error: &ErrorFromMsgHandling,
    ) -> Result<(), ErrorFromErrorHandling>;

pub type ThreadboundMessageReceiver<T, S, E> =
    fn(thread_rx: &Receiver<T>, state: &mut S) -> Result<T, E>;

pub struct PhantomHolder<T, G, S, E, EE, EEE> {
    _phantom_t: PhantomData<T>,
    _phantom_g: PhantomData<G>,
    _phantom_s: PhantomData<S>,
    _phantom_e: PhantomData<E>,
    _phantom_ee: PhantomData<EE>,
    _phantom_eee: PhantomData<EEE>,
}
unsafe impl<T, G, S, E, EE, EEE> Send for PhantomHolder<T, G, S, E, EE, EEE> {}
unsafe impl<T, G, S, E, EE, EEE> Sync for PhantomHolder<T, G, S, E, EE, EEE> {}
impl<T, G, S, E, EE, EEE> Clone for PhantomHolder<T, G, S, E, EE, EEE> {
    fn clone(&self) -> Self {
        PhantomHolder {
            _phantom_t: PhantomData,
            _phantom_g: PhantomData,
            _phantom_s: PhantomData,
            _phantom_e: PhantomData,
            _phantom_ee: PhantomData,
            _phantom_eee: PhantomData,
        }
    }
}
impl<T, G, S, E, EE, EEE> Default for PhantomHolder<T, G, S, E, EE, EEE> {
    fn default() -> Self {
        PhantomHolder {
            _phantom_t: PhantomData::<T>,
            _phantom_g: PhantomData::<G>,
            _phantom_s: PhantomData::<S>,
            _phantom_e: PhantomData::<E>,
            _phantom_ee: PhantomData::<EE>,
            _phantom_eee: PhantomData::<EEE>,
        }
    }
}

#[derive(Resource, Reflect)]
pub struct WorkerConfig<
    T,
    G,
    S,
    ErrorFromMessageHandling,
    ErrorFromErrorHandling,
    ErrorFromMessageReceiving,
> {
    pub name: String,
    pub sleep_duration: std::time::Duration,
    pub is_ui_automation_thread: bool,
    pub threadbound_message_receiver: ThreadboundMessageReceiver<T, S, ErrorFromMessageReceiving>,
    pub handle_threadbound_message: ThreadboundMessageHandler<T, G, S, ErrorFromMessageHandling>,
    pub handle_threadbound_message_error_handler:
        ThreadboundMessageErrorHandler<T, G, S, ErrorFromMessageHandling, ErrorFromErrorHandling>,
    pub gamebound_channel_capacity: usize,
    pub threadbound_channel_capacity: usize,
    pub type_holder: PhantomHolder<
        T,
        G,
        S,
        ErrorFromMessageHandling,
        ErrorFromErrorHandling,
        ErrorFromMessageReceiving,
    >,
}
impl<T, G, S> Default for WorkerConfig<T, G, S, anyhow::Error, anyhow::Error, anyhow::Error>
where
    T: WorkerMessage,
    G: WorkerMessage,
    S: WorkerState,
{
    fn default() -> Self {
        WorkerConfig {
            name: "Unknown Worker".to_string(),
            is_ui_automation_thread: false,
            sleep_duration: std::time::Duration::ZERO,
            handle_threadbound_message: |_, _, _| Ok(()),
            handle_threadbound_message_error_handler: |_, _, _, _| Ok(()),
            threadbound_message_receiver: |thread_rx, _state| {
                thread_rx
                    .recv()
                    .map_err(|e| anyhow::Error::from(e).context("receiving threadbound message"))
            },
            gamebound_channel_capacity: 10,
            threadbound_channel_capacity: 10,
            type_holder: PhantomHolder::<T, G, S, _, _, _>::default(),
        }
    }
}
impl<T, G, S, E, EE, EEE> Clone for WorkerConfig<T, G, S, E, EE, EEE> {
    fn clone(&self) -> Self {
        WorkerConfig {
            name: self.name.clone(),
            sleep_duration: self.sleep_duration,
            is_ui_automation_thread: self.is_ui_automation_thread,
            threadbound_message_receiver: self.threadbound_message_receiver,
            handle_threadbound_message: self.handle_threadbound_message,
            handle_threadbound_message_error_handler: self.handle_threadbound_message_error_handler,
            gamebound_channel_capacity: self.gamebound_channel_capacity,
            threadbound_channel_capacity: self.threadbound_channel_capacity,
            type_holder: self.type_holder.clone(),
        }
    }
}
