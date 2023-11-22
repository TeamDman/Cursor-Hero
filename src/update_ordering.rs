use bevy::ecs::schedule::SystemSet;
use bevy::prelude::*;

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
                MovementSet::AfterMovement,
            )
                .chain(),
        );
    }
}
