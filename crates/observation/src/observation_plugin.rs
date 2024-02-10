use bevy::prelude::*;

use crate::observation_buffer_plugin::ObservationBufferPlugin;
use crate::observation_log_plugin::ObservationLogPlugin;
use crate::observation_tool_plugin::ObservationToolPlugin;
use crate::observe_chat_plugin::ObserveChatPlugin;

pub struct ObservationPlugin;

impl Plugin for ObservationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ObservationLogPlugin);
        app.add_plugins(ObservationToolPlugin);
        app.add_plugins(ObservationBufferPlugin);
        app.add_plugins(ObserveChatPlugin);
    }
}
