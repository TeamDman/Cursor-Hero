use bevy::prelude::*;

pub struct VersionPlugin(pub String);

impl Plugin for VersionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Version(self.0.clone()));
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Version(pub String);
