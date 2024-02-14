use bevy::prelude::*;
use cursor_hero_voice_to_text_types::voice_to_text_types::VoiceToTextStatus;

use crate::voice_to_text_button_plugin::VoiceToTextButtonPlugin;
use crate::voice_to_text_inference_plugin::VoiceToTextInferencePlugin;
use crate::voice_to_text_ping_plugin::VoiceToTextPingPlugin;
use crate::voice_to_text_status_worker_plugin::VoiceToTextStatusWorkerPlugin;

pub struct VoiceToTextPlugin;

impl Plugin for VoiceToTextPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoiceToTextStatus>();
        app.add_plugins(VoiceToTextButtonPlugin);
        app.add_plugins(VoiceToTextInferencePlugin);
        app.add_plugins(VoiceToTextPingPlugin);
        app.add_plugins(VoiceToTextStatusWorkerPlugin);
    }
}
