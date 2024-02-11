use bevy::prelude::*;

use crate::ollama_inference_plugin::OllamaInferencePlugin;
use crate::ollama_button_plugin::OllamaButtonPlugin;
use crate::ollama_status_plugin::OllamaStatusPlugin;
use crate::ollama_status_worker_plugin::OllamaStatusWorkerPlugin;

pub struct OllamaPlugin;

impl Plugin for OllamaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OllamaInferencePlugin);
        app.add_plugins(OllamaButtonPlugin);
        app.add_plugins(OllamaStatusPlugin);
        app.add_plugins(OllamaStatusWorkerPlugin);
    }
}
