use crate::agent_movement_plugin::AgentMovementPlugin;
use crate::insert_agent_toolbelt::InsertAgentToolbeltPlugin;
use crate::spawn_agent_plugin::SpawnAgentPlugin;
use bevy::prelude::*;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(InsertAgentToolbeltPlugin);
        // app.add_plugins(SpawnAgentPlugin);
        // app.add_plugins(AgentMovementPlugin);
    }
}
