
use bevy::prelude::*;

pub struct PhysicsDebugPlugin;

impl Plugin for PhysicsDebugPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        // side effect: enabling this will cause tools to spawn visible instead of hidden
        // app.add_plugins(bevy_xpbd_2d::plugins::PhysicsDebugPlugin::default());
    }
}