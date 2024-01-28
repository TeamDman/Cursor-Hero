use bevy::prelude::*;

use bevy_xpbd_2d::math::Vector;
use bevy_xpbd_2d::plugins::setup::Physics;
use bevy_xpbd_2d::plugins::sync::SyncConfig;
use bevy_xpbd_2d::plugins::PhysicsPlugins;
use bevy_xpbd_2d::resources::Gravity;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity(Vector::ZERO))
            .insert_resource(Time::new_with(Physics::fixed_hz(144.0)))
            .insert_resource(SyncConfig {
                position_to_transform: true,
                transform_to_position: true,
            });

        // side effect: enabling this will cause tools to spawn visible instead of hidden
        // app.add_plugins(bevy_xpbd_2d::plugins::PhysicsDebugPlugin::default());
    }
}
