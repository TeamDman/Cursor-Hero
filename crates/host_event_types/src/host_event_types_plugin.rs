use crate::prelude::*;
use bevy::prelude::*;

pub struct HostEventTypesPlugin;

impl Plugin for HostEventTypesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HostEvent>();
    }
}
