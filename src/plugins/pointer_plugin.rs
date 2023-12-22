use bevy::prelude::*;
use bevy_xpbd_2d::constraints::DistanceJoint;
use bevy_xpbd_2d::{math::*, prelude::*};
use leafwing_input_manager::prelude::*;

use super::character_plugin::{Character, CharacterSystemSet, PlayerAction};

pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pointer>()
            .add_systems(
                Startup,
                (apply_deferred, setup.after(CharacterSystemSet::Spawn)).chain(),
            )
            .add_systems(Update, (apply_movement, apply_movement_damping).chain());
    }
}

#[derive(Component, Reflect, Debug)]
pub struct Pointer {
    character_id: Entity,
    speed: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character: Query<(Entity, &Transform, &Character, &Name)>,
) {
    assert!(character.iter().count() > 0, "No characters found");
    for (character_entity, transform, _character, name) in character.iter() {
        info!("Creating pointer for character '{}'", name.as_str());
        let pointer_entity = commands
            .spawn((
                Pointer {
                    character_id: character_entity.clone(),
                    speed: 100_000.0,
                },
                Name::new(format!("Pointer for {}", name.as_str())),
                SpriteBundle {
                    transform: Transform::from_translation(
                        transform.translation + Vec3::new(200.0, 0.0, 0.5),
                    ),
                    texture: asset_server.load("textures/cursor.png"),
                    sprite: Sprite {
                        color: Color::rgb(0.2, 0.7, 0.9),
                        ..default()
                    },
                    ..Default::default()
                },
                RigidBody::Dynamic,
                MassPropertiesBundle::new_computed(&Collider::cuboid(10.0, 10.0), 1.0),
            ))
            .id();

        info!(
            "Spawning pointer distance joint for character '{}'",
            name.as_str()
        );
        commands.spawn(
            DistanceJoint::new(character_entity, pointer_entity)
                .with_local_anchor_1(Vector::ZERO)
                .with_local_anchor_2(Vector::ZERO)
                .with_rest_length(200.0)
                .with_linear_velocity_damping(0.1)
                .with_angular_velocity_damping(1.0)
                // .with_limits(30.0, 300.0)
                .with_compliance(0.00000001),
        );
    }
}

fn apply_movement(
    time: Res<Time>,
    character_query: Query<&ActionState<PlayerAction>, (With<Character>, Without<Pointer>)>,
    mut pointer_query: Query<(&mut LinearVelocity, &Pointer), Without<Character>>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for (mut p_vel, p) in pointer_query.iter_mut() {
        if let Ok(c_act) = character_query.get(p.character_id) {   
            if c_act.pressed(PlayerAction::Look) {
                let look = c_act
                    .axis_pair(PlayerAction::Look)
                    .unwrap()
                    .xy();
                if look.x.is_nan() || look.y.is_nan() {
                    continue;
                }
                println!("look: {:?}, delta_time: {:?}, speed: {:?}", look, delta_time, p.speed);
                p_vel.x = look.x * delta_time * p.speed;
                p_vel.y = look.y * delta_time * p.speed;
            }
        }
    }
}

fn apply_movement_damping(
    mut query: Query<
        (&mut LinearVelocity, &mut AngularVelocity),
        (With<Pointer>, Without<Sleeping>),
    >,
    time: Res<Time<Physics>>,
) {
    if time.is_paused() {
        return;
    }
    let damping_factor = 0.95;
    for (mut linear_velocity, mut angular_velocity) in &mut query {
        linear_velocity.x *= damping_factor;
        if linear_velocity.x.abs() < 0.001 {
            linear_velocity.x = 0.0;
        }
        linear_velocity.y *= damping_factor;
        if linear_velocity.y.abs() < 0.001 {
            linear_velocity.y = 0.0;
        }
        angular_velocity.0 *= damping_factor;
        if angular_velocity.0.abs() < 0.001 {
            angular_velocity.0 = 0.0;
        }
    }
}
