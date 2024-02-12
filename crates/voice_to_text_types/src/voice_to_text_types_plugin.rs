use bevy::prelude::*;
use crate::prelude::*;

pub struct VoiceToTextTypesPlugin;

impl Plugin for VoiceToTextTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VoiceToTextStatus>();
        app.register_type::<VoiceToTextStatusButton>();
        app.register_type::<VoiceToTextStatusButtonVisualState>();
        app.register_type::<VoiceToTextStatusEvent>();
        app.add_event::<VoiceToTextStatusEvent>();

        app.register_type::<VoiceToTextPingEvent>();
        app.add_event::<VoiceToTextPingEvent>();

        app.register_type::<VoiceToTextVscodeButton>();
        app.register_type::<VoiceToTextVscodeButtonVisualState>();
        app.register_type::<VoiceToTextVscodeEvent>();
        app.add_event::<VoiceToTextVscodeEvent>();
    }
}