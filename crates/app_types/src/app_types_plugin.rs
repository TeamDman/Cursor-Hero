use crate::prelude::*;
use bevy::prelude::*;

pub struct AppTypesPlugin;

impl Plugin for AppTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyComponent>();
        app.add_event::<MyEvent>();
    }
}
