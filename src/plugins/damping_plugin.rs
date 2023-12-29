use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_2d::{math::*, prelude::*};

pub struct DampingPlugin;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum DampingSystemSet {
    Dampen,
}

impl Plugin for DampingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MovementDamping>().add_systems(
            Update,
            apply_movement_damping.in_set(DampingSystemSet::Dampen),
        );
    }
}

/// The damping factor used for slowing down movement.
#[derive(Component, Reflect, InspectorOptions, Debug)]
#[reflect(Component, InspectorOptions)]
pub struct MovementDamping {
    #[inspector(min = 0.5, max = 0.999)]
    pub factor: Scalar,
}

impl Default for MovementDamping {
    fn default() -> Self {
        Self { factor: 0.95 }
    }
}

#[allow(clippy::type_complexity)]
fn apply_movement_damping(
    mut query: Query<
        (&MovementDamping, &mut LinearVelocity, &mut AngularVelocity),
        Without<Sleeping>,
    >,
    time: Res<Time<Physics>>,
) {
    if time.is_paused() {
        return;
    }
    for (damping, mut linear_velocity, mut angular_velocity) in &mut query {
        linear_velocity.x *= damping.factor;
        if linear_velocity.x.abs() < 10.0 {
            linear_velocity.x = 0.0;
        }
        linear_velocity.y *= damping.factor;
        if linear_velocity.y.abs() < 10.0 {
            linear_velocity.y = 0.0;
        }
        angular_velocity.0 *= damping.factor;
        if angular_velocity.0.abs() < 10.0 {
            angular_velocity.0 = 0.0;
        }
    }
}
