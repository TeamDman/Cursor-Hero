use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use super::character_plugin::{Character, CharacterSystemSet, PlayerAction};

pub struct PointerPlugin;
#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum PointerSystemSet {
    Position,
}

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pointer>()
        .configure_sets(Update, PointerSystemSet::Position)
            .add_systems(
                Startup,
                (apply_deferred, setup.after(CharacterSystemSet::Spawn)).chain(),
            )
            .add_systems(
                PostUpdate,
                update_pointer_position
                    .in_set(PointerSystemSet::Position)
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Component, Reflect, Debug)]
pub struct Pointer;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character: Query<Entity, With<Character>>,
) {
    assert!(character.iter().count() > 0, "No characters found");
    for c_e in character.iter() {
        info!("Creating pointer for character '{:?}'", c_e);
        commands.entity(c_e).with_children(|c_commands| {
            c_commands.spawn((
                Pointer,
                Name::new("Pointer"),
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(200.0, 0.0, 0.5)),
                    texture: asset_server.load("textures/cursor.png"),
                    sprite: Sprite {
                        color: Color::rgb(0.2, 0.7, 0.9),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    ..Default::default()
                },
                RigidBody::Dynamic,
                MassPropertiesBundle::new_computed(&Collider::cuboid(10.0, 10.0), 1.0),
            ));
        });
    }
}

fn update_pointer_position(
    character_query: Query<&ActionState<PlayerAction>, With<Character>>,
    mut pointer_query: Query<(&mut Transform, &Parent), With<Pointer>>,
) {
    for (mut pointer_transform, p_parent) in pointer_query.iter_mut() {
        if let Ok(c_act) = character_query.get(p_parent.get()) {
            if c_act.pressed(PlayerAction::Look) {
                let look = c_act.axis_pair(PlayerAction::Look).unwrap().xy();
                if look.x.is_nan() || look.y.is_nan() {
                    continue;
                }

                let desired_position = look.extend(0.0) * 200.0;
                pointer_transform.translation = desired_position;
            }
        }
    }
}
