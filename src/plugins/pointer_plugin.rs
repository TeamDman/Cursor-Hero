use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::prelude::*;
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
            .add_systems(
                PostUpdate,
                apply_movement
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
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
    }
}

fn apply_movement(
    character_query: Query<(&Transform, &ActionState<PlayerAction>), With<Character>>,
    mut pointer_query: Query<(&mut Transform, &Pointer), Without<Character>>,
) {
    for (mut pointer_transform, p) in pointer_query.iter_mut() {
        if let Ok((character_transform, c_act)) = character_query.get(p.character_id) {
            if c_act.pressed(PlayerAction::Look) {
                let look = c_act.axis_pair(PlayerAction::Look).unwrap().xy();
                if look.x.is_nan() || look.y.is_nan() {
                    continue;
                }

                let desired_position = character_transform.translation + look.extend(0.0) * 200.0; // * p.distance;

                pointer_transform.translation = desired_position;
            }
        }
    }
}
