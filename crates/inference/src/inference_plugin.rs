use bevy::prelude::*;

use crate::ollama_plugin::OllamaPlugin;

pub struct InferencePlugin;

impl Plugin for InferencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OllamaPlugin);
    }
}
