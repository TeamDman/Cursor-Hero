use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct PointerEnvironment {
    pub environment_id: Entity,
}
