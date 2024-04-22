use bevy::prelude::*;

#[derive(Component, Debug, Reflect, Clone, Copy)]
pub enum EnvironmentKind {
    Host,
    Agent,
}
impl EnvironmentKind {
    pub fn name(&self) -> &str {
        match self {
            EnvironmentKind::Host => "Host Environment",
            EnvironmentKind::Agent => "Agent Environment",
        }
    }
}

#[derive(Component, Debug, Reflect)]
pub struct ShouldTrackEnvironment;

#[derive(Component, Debug, Reflect, Eq, PartialEq)]
pub struct TrackedEnvironment {
    pub environment_id: Entity,
}
#[derive(Component, Debug, Reflect)]
pub struct HostEnvironment;
#[derive(Component, Debug, Reflect)]
pub struct AgentEnvironment;

#[derive(Component, Default, Reflect)]
pub struct Nametag;

#[derive(Event, Debug, Reflect)]
pub struct CreateEnvironmentRequestEvent {
    pub origin: Vec2,
    pub kind: EnvironmentKind,
}

#[derive(Event, Debug, Reflect)]
pub struct PopulateEnvironmentEvent {
    pub environment_id: Entity,
}

#[derive(Event, Debug, Reflect)]
pub enum NametagEvent {
    Update {
        environment_id: Entity,
        name: String,
    },
    RecalculatePosition {
        environment_id: Entity,
    },
}
