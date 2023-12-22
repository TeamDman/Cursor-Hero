use bevy::transform::TransformSystem;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::utils::win_mouse::set_cursor_position;
use crate::utils::win_window::{get_title_bar_height, get_window_bounds_from_title};

use super::character_plugin::{Character, CharacterSystemSet, PlayerAction};

pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pointer>()
            .register_type::<SnapMouseToPointer>()
            .add_systems(
                Startup,
                (apply_deferred, setup.after(CharacterSystemSet::Spawn)).chain(),
            )
            .add_systems(
                PostUpdate,
                (
                    apply_movement,
                    snap_mouse_to_pointer.run_if(should_snap_mouse),
                )
                    .chain()
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

#[derive(Component, Reflect, Debug)]
pub struct SnapMouseToPointer;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character: Query<(Entity, &Transform, &Character, &Name)>,
) {
    assert!(character.iter().count() > 0, "No characters found");
    for (i, (character_entity, transform, _character, name)) in character.iter().enumerate() {
        info!("Creating pointer for character '{}'", name.as_str());
        let mut pointer = commands.spawn((
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
        ));
        if i == 0 {
            pointer.insert(SnapMouseToPointer);
        }
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
                                                                                                   // debug!("look: {:?}, desired_position: {:?}", look, desired_position);
                pointer_transform.translation = desired_position;
            }
        }
    }
}

fn should_snap_mouse(
    character: Query<Ref<GlobalTransform>, With<Character>>,
    pointer: Query<(&Pointer, Ref<GlobalTransform>), With<SnapMouseToPointer>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut ready: Local<bool>,
) -> bool {
    if !*ready
        && window_query
            .get_single()
            .map(|window| get_window_bounds_from_title(window.title.as_str()).is_ok())
            .unwrap_or(false)
    {
        *ready = true;
    }
    if !*ready {
        return false;
    }
    if let Ok((p, p_pos)) = pointer.get_single() {
        if let Ok(c_pos) = character.get(p.character_id) {
            return p_pos.is_changed() || c_pos.is_changed();
        }
    }
    return false;
}
fn snap_mouse_to_pointer(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    pointer_query: Query<&GlobalTransform, With<SnapMouseToPointer>>,
) {
    let window = window_query.get_single().expect("Need a single window");
    let window_position =
        get_window_bounds_from_title(window.title.as_str()).expect("Need a window position");

    let (camera_transform, camera) = camera_query.get_single().expect("Need a single camera");
    let pointer = pointer_query.get_single().expect("Need a single pointer");
    let pointer_position = pointer.translation();
    if let Some(viewport_position) = camera.world_to_viewport(camera_transform, pointer_position) {
        let mut pos: Vec2 = Vec2::ZERO;
        pos.x += window_position.left as f32 + viewport_position.x;
        pos.y += window_position.top as f32 + viewport_position.y;
        pos.y += get_title_bar_height() as f32;
        // pos.y *= -1.0;
        // println!(
        //     "window: {:?}, viewport: {:?}, pointer: {:?}",
        //     window_position, viewport_position, pos
        // );
        let _ = set_cursor_position(pos.x as i32, pos.y as i32);
    }
}
