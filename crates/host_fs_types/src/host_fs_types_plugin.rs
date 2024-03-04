use crate::prelude::*;
use bevy::prelude::*;

pub struct HostFsTypesPlugin;

impl Plugin for HostFsTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HostPath>();
        app.add_event::<HostPathAction>();
    }
}
