use bevy::prelude::*;
use crate::{insert_agent_toolbelt::InsertAgentToolbeltPlugin, spawn_agent_plugin::SpawnAgentPlugin};

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InsertAgentToolbeltPlugin);
        app.add_plugins(SpawnAgentPlugin);
    }
}
