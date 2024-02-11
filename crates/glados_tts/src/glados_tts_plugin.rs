use bevy::prelude::*;

use crate::glados_tts_button_plugin::GladosTtsButtonPlugin;
use crate::glados_tts_inference_plugin::GladosTtsInferencePlugin;
use crate::glados_tts_status_plugin::GladosTtsStatusPlugin;
use crate::glados_tts_status_worker_plugin::GladosTtsStatusWorkerPlugin;

pub struct GladosTtsPlugin;

impl Plugin for GladosTtsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GladosTtsInferencePlugin);
        app.add_plugins(GladosTtsButtonPlugin);
        app.add_plugins(GladosTtsStatusPlugin);
        app.add_plugins(GladosTtsStatusWorkerPlugin);
    }
}
