use bevy::prelude::*;

pub struct EnvironmentTypesPlugin;

impl Plugin for EnvironmentTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyComponent>();
        app.add_event::<MyEvent>();
    }
}