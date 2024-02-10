use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub struct InferenceSession;

#[derive(Event, Reflect, Debug, Clone)]
pub enum InferenceEvent {
    Request {
        session_id: Entity,
        prompt: Prompt,
    },
    Response {
        session_id: Entity,
        prompt: Prompt,
        response: String,
    },
}
