use bevy::ecs::schedule::SystemSet;
use bevy::prelude::*;
use bevy_xpbd_2d::PhysicsSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum MovementSet {
    Input,
    PrePhysics,
    AfterMovement,
}

pub struct UpdateOrderingPlugin;

impl Plugin for UpdateOrderingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                MovementSet::Input,
                MovementSet::PrePhysics,
                PhysicsSet::StepSimulation,
                MovementSet::AfterMovement,
            )
                .chain(),
        );
    }
}
