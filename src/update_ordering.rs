use bevy::ecs::schedule::SystemSet;
use bevy::prelude::*;
use bevy_xpbd_2d::PhysicsSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum MovementSet {
    Input,
    AfterMovement,
}

pub struct UpdateOrderingPlugin;

impl Plugin for UpdateOrderingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                MovementSet::Input,
                PhysicsSet::Sync,
                MovementSet::AfterMovement,
            )
                .chain(),
        );
    }
}
