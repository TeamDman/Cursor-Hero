use bevy::{
    prelude::*,
    transform::TransformSystem,
    window::{PrimaryWindow, RawHandleWrapper},
};
use bevy_xpbd_2d::prelude::*;
use itertools::Itertools;

use crate::{
    plugins::{
        character_plugin::Character,
        pointer_plugin::{Pointer, PointerSystemSet},
    },
    utils::{
        win_mouse::set_cursor_position,
        win_window::{get_window_bounds, get_window_inner_offset},
    },
};

use super::super::toolbelt::types::*;

pub struct PointerWindowPositionToolPlugin;

impl Plugin for PointerWindowPositionToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PointerWindowPositionTool>()
            .add_systems(Update, spawn_tool_event_responder_update_system)
            .add_systems(
                PostUpdate,
                snap_mouse_to_pointer
                    .after(PointerSystemSet::Position)
                    .after(PhysicsSet::Sync)
                    .after(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Component, Reflect)]
pub struct PointerWindowPositionTool;

fn spawn_tool_event_responder_update_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            ToolbeltEvent::Populate(toolbelt_id) => {
                commands.entity(*toolbelt_id).with_children(|t_commands| {
                    t_commands.spawn((
                        ToolBundle {
                            name: Name::new("Pointer Window Position Tool"),
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server.load("textures/tool_ripples.png"),
                                ..default()
                            },
                            ..default()
                        },
                        PointerWindowPositionTool,
                    ));
                });
                info!("Added click tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

fn snap_mouse_to_pointer(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&RawHandleWrapper, With<PrimaryWindow>>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<(Ref<GlobalTransform>, &Children), With<Character>>,
    pointers: Query<Ref<GlobalTransform>, With<Pointer>>,
    tools: Query<(Option<&ToolActiveTag>, &Parent), With<PointerWindowPositionTool>>,
) {
    // ensure only a single cursor positioning tool is active
    let active = tools
        .iter()
        .filter(|(t_active, _)| t_active.is_some())
        .collect_vec();
    let active_count = active.len();
    if active_count > 1 {
        warn!("Only one cursor positioning tool should be active at a time");
    }
    if active_count == 0 {
        return;
    }

    // get the pointer position
    let (c_pos, c_kids) = characters
        .get(
            toolbelts
                .get(active.first().unwrap().1.get())
                .expect("Toolbelt should have a parent")
                .get(),
        )
        .expect("Toolbelt should have a character");
    let p_pos = c_kids
        .iter()
        .filter_map(|x| pointers.get(*x).ok())
        .next()
        .expect("Character should have a pointer");

    // ensure a change has occurred
    if !p_pos.is_changed() && !c_pos.is_changed() {
        return;
    }

    let window_handle = window_query.get_single().expect("Need a single window");
    let win32handle = match window_handle.window_handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => handle,
        _ => panic!("Unsupported window handle"),
    };
    let window_position = get_window_bounds(win32handle.hwnd as _).expect("Need a window position");

    let (camera_transform, camera) = camera_query.get_single().expect("Need a single camera");
    if let Some(viewport_position) = camera.world_to_viewport(camera_transform, p_pos.translation())
    {
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
