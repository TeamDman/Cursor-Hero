use crate::prelude::*;
use bevy::prelude::*;

pub struct EnvironmentTypesPlugin;

impl Plugin for EnvironmentTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ShouldTrackEnvironment>();
        app.register_type::<TrackedEnvironment>();
        app.register_type::<EnvironmentKind>();
        app.register_type::<HostEnvironment>();
        app.register_type::<AgentEnvironment>();
        app.register_type::<Nametag>();
        app.add_event::<CreateEnvironmentRequestEvent>();
        app.add_event::<PopulateEnvironmentEvent>();
        app.add_event::<NametagEvent>();
    }
}
