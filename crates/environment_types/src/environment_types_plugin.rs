use bevy::prelude::*;

pub struct EnvironmentTypesPlugin;

impl Plugin for EnvironmentTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Environment>();
        app.register_type::<HostEnvironment>();
        app.add_event::<CreateEnvironmentEvent>();
        app.add_event::<PopulateEnvironmentEvent>();
    }
}