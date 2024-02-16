use crate::prelude::*;
use bevy::prelude::*;

pub struct OllamaTypesPlugin;

impl Plugin for OllamaTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OllamaStatus>();
        app.register_type::<OllamaStatusButton>();
        app.register_type::<OllamaStatusButtonVisualState>();
        app.register_type::<OllamaStatusEvent>();
        app.add_event::<OllamaStatusEvent>();

        app.register_type::<OllamaPingEvent>();
        app.add_event::<OllamaPingEvent>();

        app.register_type::<OllamaTerminalButton>();
        app.register_type::<OllamaTerminalButtonVisualState>();
        app.register_type::<OllamaTerminalEvent>();
        app.add_event::<OllamaTerminalEvent>();
    }
}
