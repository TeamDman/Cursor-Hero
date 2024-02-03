use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub struct Agent;

#[derive(Reflect, Eq, PartialEq, Debug)]
pub enum AgentAppearance {
    Default,
}
impl AgentAppearance {
    pub fn get_texture_path(&self) -> &'static str {
        match self {
            Self::Default => "textures/agent/default_agent.png",
        }
    }
}
