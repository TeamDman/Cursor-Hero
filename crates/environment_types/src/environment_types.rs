use bevy::prelude::*;

#[derive(Component, Debug, Reflect, Clone, Copy)]
pub enum EnvironmentKind {
    Host,
    HostUIWatcher,
    Game,
}

#[derive(Component, Debug, Reflect, Eq, PartialEq)]
pub struct EnvironmentTracker {
    pub environment_id: Entity,
}

#[derive(Component, Debug, Reflect)]
pub struct HostEnvironment;
#[derive(Component, Debug, Reflect)]
pub struct HostUIWatcherEnvironment;
#[derive(Component, Debug, Reflect)]
pub struct GameEnvironment;

#[derive(Component, Default, Reflect)]
pub struct Nametag;

#[derive(Component, Debug, Reflect)]
pub struct TrackEnvironmentTag;

#[derive(Event, Debug, Reflect)]
pub struct CreateEnvironmentRequestEvent {
    pub name: String,
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
