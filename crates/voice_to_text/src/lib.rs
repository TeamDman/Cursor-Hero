#![feature(let_chains)]
pub mod voice_to_text_plugin;
pub mod voice_to_text_button_plugin;
pub mod voice_to_text;
pub mod voice_to_text_ping_plugin;
pub mod voice_to_text_worker_plugin;

pub mod prelude {
    pub use crate::voice_to_text_plugin::*;
}