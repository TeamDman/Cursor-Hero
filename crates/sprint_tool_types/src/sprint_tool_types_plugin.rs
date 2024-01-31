use bevy::prelude::*;

pub struct SprintToolTypesPlugin;

impl Plugin for SprintToolTypesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SprintEvent>();
    }
}

#[derive(Event, Debug, Reflect)]
pub enum SprintEvent {
    Active { character_id: Entity, throttle: f32 },
    Stop { character_id: Entity },
}