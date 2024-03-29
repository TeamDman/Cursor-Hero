use crate::prelude::*;
use bevy::prelude::*;

#[derive(Reflect, Debug, Clone, Default, Eq, PartialEq)]
pub struct TextInferenceOptions {
    pub num_predict: Option<usize>,
    pub stop: Option<Vec<String>>,
}

#[derive(Event, Reflect, Debug, Clone)]
pub enum TextInferenceEvent {
    Request {
        session_id: Entity,
        prompt: TextPrompt,
    },
    Response {
        session_id: Entity,
        prompt: MaterializedTextPrompt,
        response: String,
    },
}

#[derive(Event, Reflect, Debug, Clone)]
pub enum SpeechInferenceEvent {
    Request {
        session_id: Entity,
        prompt: SpeechPrompt,
    },
    Response {
        session_id: Entity,
        prompt: SpeechPrompt,
        wav: Vec<u8>,
    },
}

#[derive(Event, Reflect, Debug, Clone)]
pub enum TranscriptionInferenceEvent {
    Request {
        session_id: Entity,
        prompt: TranscriptionPrompt,
    },
    Response {
        session_id: Entity,
        prompt: TranscriptionPrompt,
        transcription: String,
    },
}
