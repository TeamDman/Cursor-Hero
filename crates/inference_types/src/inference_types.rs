use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub struct InferenceSession;

#[derive(Event, Reflect, Debug, Clone)]
pub enum InferenceEvent {
    GenerateRequest {
        session_id: Entity,
        prompt: String,
    },
    GenerateResponse {
        session_id: Entity,
        response: String,
    },
}