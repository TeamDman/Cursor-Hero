use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub struct InferenceSession;

#[derive(Event, Reflect, Debug, Clone)]
pub enum InferenceEvent {
    Request {
        session_id: Entity,
        prompt: String,
    },
    Response {
        session_id: Entity,
        prompt: String,
        response: String,
    },
}