use crate::prelude::*;
use bevy::prelude::*;

pub struct EnvironmentTypesPlugin;

impl Plugin for EnvironmentTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TrackEnvironmentTag>();
        app.register_type::<EnvironmentTracker>();
        app.register_type::<DoNotTrackEnvironment>();
        app.register_type::<EnvironmentKind>();
        app.register_type::<HostEnvironment>();
        app.register_type::<AgentEnvironment>();
        app.register_type::<Nametag>();
        app.add_event::<CreateEnvironmentRequestEvent>();
        app.add_event::<PopulateEnvironmentEvent>();
        app.add_event::<NametagEvent>();
    }
}
