use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
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
    std::fmt::Debug + GetTypeRegistration + Event + Send + Sync + Clone + Reflect + TypePath + FromReflect + 'static
{
}

#[derive(Resource, Reflect, Clone)]
pub struct WorkerConfig<T, G> {
    pub name: String,
    pub sleep_duration: std::time::Duration,
    pub is_ui_automation_thread: bool,
    pub handle_threadbound_message:
        fn(msg: &T, reply_tx: &Sender<G>) -> Result<(), Box<dyn std::error::Error>>,
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
            _phantom_t: PhantomData,
            _phantom_g: PhantomData,
        }
    }
}
