pub mod ollama;
pub mod ollama_button_plugin;
pub mod ollama_inference_plugin;
pub mod ollama_plugin;
pub mod ollama_status_plugin;
pub mod ollama_status_worker_plugin;

pub mod prelude {
    pub use crate::ollama_plugin::*;
}
