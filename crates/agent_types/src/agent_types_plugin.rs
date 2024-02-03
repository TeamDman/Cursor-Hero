use bevy::prelude::*;

use crate::prelude::*;

pub struct AgentTypesPlugin;

impl Plugin for AgentTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Agent>();
    }
}
