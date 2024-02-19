use bevy::prelude::*;
use crate::prelude::*;

pub struct AppTypesPlugin;

impl Plugin for AppTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyComponent>();
        app.add_event::<MyEvent>();
    }
}