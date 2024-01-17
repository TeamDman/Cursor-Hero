use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_winutils::win_window::ToBevyRect;
use itertools::Itertools;

use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_glam::bevy::NegativeY;
use cursor_hero_pointer::pointer_plugin::Pointer;
use cursor_hero_pointer::pointer_plugin::PointerSystemSet;
use cursor_hero_toolbelt::types::*;
use cursor_hero_winutils::win_mouse::set_cursor_position;
use cursor_hero_winutils::win_window::get_window_bounds;
use cursor_hero_winutils::win_window::get_window_inner_offset;

use crate::prelude::*;

pub struct CursorMonitorPositionToolPlugin;

impl Plugin for CursorMonitorPositionToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CursorMonitorPositionTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(
                PostUpdate,
                snap_mouse_to_pointer
                    .run_if(in_state(ActiveInput::Gamepad))
                    .after(PointerSystemSet::Position)
                    .after(PhysicsSet::Sync)
                    .after(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Component, Reflect)]
struct CursorMonitorPositionTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            ToolbeltEvent::PopulateDefaultToolbelt {
                toolbelt_id,
                character_id,
            } => {
                spawn_tool(
                    file!(),
                    e,
                    &mut commands,
                    *toolbelt_id,
                    *character_id,
                    &asset_server,
                    CursorMonitorPositionTool,
                );
            }
            _ => {}
        }
    }
}

fn snap_mouse_to_pointer(
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<(Ref<GlobalTransform>, &Children), With<Character>>,
    pointers: Query<Ref<GlobalTransform>, With<Pointer>>,
    tools: Query<(Option<&ActiveTool>, &Parent), With<CursorMonitorPositionTool>>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&RawHandleWrapper, With<PrimaryWindow>>,
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
    let (character_position, character_children) = characters
        .get(
            toolbelts
                .get(active.first().unwrap().1.get())
                .expect("Toolbelt should have a parent")
                .get(),
        )
        .expect("Toolbelt should have a character");
    let pointer_position = character_children
        .iter()
        .filter_map(|x| pointers.get(*x).ok())
        .next()
        .expect("Character should have a pointer");

    // ensure a change has occurred
    if !pointer_position.is_changed() && !character_position.is_changed() {
        return;
    }
    // debug!(
    //     "pointer changed: {}, character changed: {}",
    //     pointer_position.is_changed(),
    //     character_position.is_changed()
    // );

    let destination_position = pointer_position.translation();

    let window_handle = window_query.get_single().expect("Need a single window");
    let win32handle = match window_handle.window_handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => handle,
        _ => panic!("Unsupported window handle"),
    };
    let window_position = get_window_bounds(win32handle.hwnd as _)
        .expect("Need a window position")
        .to_bevy_rect();
    let (camera_transform, camera) = camera_query.get_single().expect("Need a single camera");
    let is_over_window = window_position.contains(destination_position.xy().neg_y());
    debug!(
        "contains: {}, window: {:?}, destination: {:?}",
        is_over_window, window_position, destination_position
    );
    if is_over_window
        && let Some(viewport_position) =
            camera.world_to_viewport(camera_transform, destination_position)
    {
        debug!("viewport position: {:?}, window position: {:?}", viewport_position, window_position);
        let mut pos: Vec2 = Vec2::ZERO;
        pos.x += window_position.min.x as f32 + viewport_position.x;
        pos.y += window_position.min.y as f32 + viewport_position.y;
        let offset = get_window_inner_offset();
        pos.x += offset.0 as f32 * 2.0;
        pos.y += offset.1 as f32;
        match set_cursor_position(pos.x as i32, pos.y as i32) {
            Ok(_) => {}
            Err(e) => warn!("Failed to set cursor position: {}", e),
        }
    } else {
        match set_cursor_position(
            destination_position.x as i32,
            -destination_position.y as i32,
        ) {
            Ok(_) => {}
            Err(e) => warn!("Failed to set cursor position: {}", e),
        }
    }
}
