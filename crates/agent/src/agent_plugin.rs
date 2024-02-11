use crate::agent_movement_plugin::AgentMovementPlugin;
use crate::agent_spawning_plugin::AgentSpawningPlugin;
use crate::insert_agent_toolbelt::InsertAgentToolbeltPlugin;
use bevy::prelude::*;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InsertAgentToolbeltPlugin);
        app.add_plugins(AgentSpawningPlugin);
        app.add_plugins(AgentMovementPlugin);
    }
}
