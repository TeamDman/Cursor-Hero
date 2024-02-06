use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub struct TtsSession;

#[derive(Event, Reflect, Debug, Clone)]
pub enum TtsEvent {
    Request {
        session_id: Entity,
        prompt: String,
    },
    Response {
        session_id: Entity,
        prompt: String,
        wav: Vec<u8>,
    },
}
