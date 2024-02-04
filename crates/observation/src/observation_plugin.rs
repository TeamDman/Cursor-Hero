use bevy::prelude::*;

use crate::observation_inference_plugin::ObservationInferencePlugin;
use crate::observation_log_plugin::ObservationLogPlugin;
use crate::observation_tool_plugin::ObservationToolPlugin;

pub struct ObservationPlugin;

impl Plugin for ObservationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ObservationLogPlugin);
        app.add_plugins(ObservationInferencePlugin);
        app.add_plugins(ObservationToolPlugin);
    }
}
