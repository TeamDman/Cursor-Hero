use bevy::sprite::Anchor;
use bevy::transform::TransformSystem;
use bevy::window::RawHandleWrapper;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::utils::win_mouse::set_cursor_position;
use crate::utils::win_window::{get_window_bounds, get_window_inner_offset};

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
                    update_pointer_position,
                    snap_mouse_to_pointer.run_if(should_snap_mouse),
                )
                    .chain()
                    .after(PhysicsSet::Sync)
                    .after(TransformSystem::TransformPropagate),
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
    character: Query<(Entity, &Transform), With<Character>>,
) {
    assert!(character.iter().count() > 0, "No characters found");
    for (i, (c_e, c_pos)) in character.iter().enumerate() {
        info!("Creating pointer for character '{:?}'", c_e);
        commands.entity(c_e).with_children(|c_commands| {
            let mut pointer = c_commands.spawn((
                Pointer {
                    character_id: c_e,
                    speed: 100_000.0,
                },
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
            if i == 0 {
                pointer.insert(SnapMouseToPointer);
            }
        });
    }
}

fn update_pointer_position(
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

                let desired_position = look.extend(0.0) * 200.0;
                pointer_transform.translation = desired_position;
            }
        }
    }
}

fn should_snap_mouse(
    character: Query<Ref<GlobalTransform>, With<Character>>,
    pointer: Query<(&Pointer, Ref<GlobalTransform>), With<SnapMouseToPointer>>,
) -> bool {
    if let Ok((p, p_pos)) = pointer.get_single() {
        if let Ok(c_pos) = character.get(p.character_id) {
            return p_pos.is_changed() || c_pos.is_changed();
        }
    }
    false
}
fn snap_mouse_to_pointer(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&RawHandleWrapper, With<PrimaryWindow>>,
    pointer_query: Query<&GlobalTransform, With<SnapMouseToPointer>>,
) {
    let window_handle = window_query.get_single().expect("Need a single window");
    let win32handle = match window_handle.window_handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => handle,
        _ => panic!("Unsupported window handle"),
    };
    let window_position = get_window_bounds(win32handle.hwnd as _).expect("Need a window position");

    let (camera_transform, camera) = camera_query.get_single().expect("Need a single camera");
    let pointer = pointer_query.get_single().expect("Need a single pointer");
    let pointer_position = pointer.translation();
    if let Some(viewport_position) = camera.world_to_viewport(camera_transform, pointer_position) {
        debug!("viewport_position: {:?}", viewport_position);
        let mut pos: Vec2 = Vec2::ZERO;
        pos.x += window_position.left as f32 + viewport_position.x;
        pos.y += window_position.top as f32 + viewport_position.y;
        let offset = get_window_inner_offset();
        pos.x += offset.0 as f32;
        pos.y += offset.1 as f32;
        // debug!("Setting cursor position to {:?}", pos);
        let result = set_cursor_position(pos.x as i32, pos.y as i32);
        if let Err(e) = result {
            warn!("Failed to set cursor position: {}", e);
        }
    }
}
