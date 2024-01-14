use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use bevy_xpbd_2d::prelude::*;
use itertools::Itertools;

use cursor_hero_character::character_plugin::Character;
use cursor_hero_pointer::pointer_plugin::Pointer;
use cursor_hero_pointer::pointer_plugin::PointerSystemSet;
use cursor_hero_toolbelt::types::*;
use cursor_hero_winutils::win_mouse::set_cursor_position;
use cursor_hero_winutils::win_window::get_window_bounds;
use cursor_hero_winutils::win_window::get_window_inner_offset;

// use crate::prelude::*;

pub struct CursorWindowPositionToolPlugin;

impl Plugin for CursorWindowPositionToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CursorWindowPositionTool>()
            .add_systems(Update, toolbelt_events)
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
struct CursorWindowPositionTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            // registration disabled for now
            _ => {}
        }
    }
}

fn snap_mouse_to_pointer(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&RawHandleWrapper, With<PrimaryWindow>>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<(Ref<GlobalTransform>, &Children), With<Character>>,
    pointers: Query<Ref<GlobalTransform>, With<Pointer>>,
    tools: Query<(Option<&ActiveTool>, &Parent), With<CursorWindowPositionTool>>,
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
