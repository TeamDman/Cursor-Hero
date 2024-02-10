use bevy::prelude::*;
use crate::prelude::*;

pub struct EnvironmentTypesPlugin;

impl Plugin for EnvironmentTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TrackEnvironmentTag>();
        app.register_type::<EnvironmentTag>();
        app.register_type::<Environment>();
        app.register_type::<HostEnvironment>();
        app.register_type::<GameEnvironment>();
        app.register_type::<Nametag>();
        app.add_event::<CreateEnvironmentEvent>();
        app.add_event::<PopulateEnvironmentEvent>();
        app.add_event::<NametagEvent>();
    }
}