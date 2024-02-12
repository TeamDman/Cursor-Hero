use bevy::prelude::*;

use crate::voice_to_text_button_plugin::VoiceToTextButtonPlugin;
use crate::voice_to_text_inference_plugin::VoiceToTextInferencePlugin;
use crate::voice_to_text_status_plugin::VoiceToTextStatusPlugin;
use crate::voice_to_text_status_worker_plugin::VoiceToTextStatusWorkerPlugin;

pub struct VoiceToTextPlugin;

impl Plugin for VoiceToTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VoiceToTextButtonPlugin);
        app.add_plugins(VoiceToTextInferencePlugin);
        app.add_plugins(VoiceToTextStatusPlugin);
        app.add_plugins(VoiceToTextStatusWorkerPlugin);
    }
}
