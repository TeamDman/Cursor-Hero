use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct Environment;
#[derive(Component, Debug, Reflect)]
pub struct HostEnvironment;
#[derive(Component, Debug, Reflect)]
pub struct GameEnvironment;

#[derive(Event, Debug, Reflect)]
pub enum CreateEnvironmentEvent {
    Host { origin: Vec2, name: String },
    Game { origin: Vec2, name: String },
}

#[derive(Event, Debug, Reflect)]
pub enum PopulateEnvironmentEvent {
    Host { environment_id: Entity },
    Game { environment_id: Entity },
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

#[derive(Component, Default, Reflect)]
pub struct Nametag;

#[derive(Component, Debug, Reflect)]
pub struct PointerEnvironment {
    pub environment_id: Entity,
}
