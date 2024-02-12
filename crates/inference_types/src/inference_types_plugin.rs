use crate::prelude::*;
use bevy::prelude::*;
pub struct InferenceTypesPlugin;

impl Plugin for InferenceTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TextPrompt>();
        app.register_type::<MaterializedTextPrompt>();
        app.register_type::<TextInferenceEvent>();
        app.add_event::<TextInferenceEvent>();

        app.register_type::<SpeechPrompt>();
        app.register_type::<SpeechInferenceEvent>();
        app.add_event::<SpeechInferenceEvent>();

        app.register_type::<TranscriptionPrompt>();
        app.register_type::<TranscriptionInferenceEvent>();
        app.add_event::<TranscriptionInferenceEvent>();
    }
}
