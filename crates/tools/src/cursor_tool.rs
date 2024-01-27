use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_glam::NegativeYI;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use itertools::Itertools;

use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_pointer::pointer_plugin::Pointer;
use cursor_hero_pointer::pointer_plugin::PointerSystemSet;
use cursor_hero_toolbelt::types::*;
use cursor_hero_winutils::win_mouse::set_cursor_position;
use cursor_hero_winutils::win_window::get_window_bounds;
use cursor_hero_winutils::win_window::get_window_inner_offset;

use crate::prelude::*;

pub struct CursorToolPlugin;

impl Plugin for CursorToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CursorTool>()
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

#[derive(Component, Reflect, Default)]
struct CursorTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        if let ToolbeltPopulateEvent::Default {
            toolbelt_id,
        }
        | ToolbeltPopulateEvent::Inspector {
            toolbelt_id,
        }
        | ToolbeltPopulateEvent::Keyboard {
            toolbelt_id,
        } = event
        {
            ToolSpawnConfig::<CursorTool, NoInputs>::new(CursorTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Positions the Windows cursor based on the game pointer")
                .spawn(&mut commands);
        }
    }
}

fn snap_mouse_to_pointer(
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<(Ref<GlobalTransform>, &Children), With<Character>>,
    pointers: Query<Ref<GlobalTransform>, With<Pointer>>,
    tools: Query<(Option<&ActiveTool>, &Parent), With<CursorTool>>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<(&RawHandleWrapper, &Window), With<PrimaryWindow>>,
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

    // get the destination position
    let mut destination_position = pointer_position.translation().xy().as_ivec2();

    // only when focused, do repositioning logic for when the cursor is over the window
    let (window_handle, window) = window_query.get_single().expect("Need a single window");

    let dragging_window = true; // TODO: get this from the window
                                // for now we will force this to true to disable the inside-window logic since it breaks repositioning the window with the controller

    if window.focused && !dragging_window {
        // get the window bounds
        let window_bounds = match window_handle.window_handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_window_bounds(handle.hwnd as _).expect("Need a window position")
            }
            _ => panic!("Unsupported window handle"),
        };

        // get the viewport position of the pointer
        let is_over_window = window_bounds.contains(destination_position.neg_y());
        let viewport_position = match is_over_window {
            true => {
                let (camera_transform, camera) =
                    camera_query.get_single().expect("Need a single camera");
                camera
                    .world_to_viewport(camera_transform, destination_position.as_vec2().extend(0.0))
            }
            false => None,
        };

        // if the pointer is in view, position the cursor _over_ the pointer instead
        if let Some(viewport_position) = viewport_position {
            destination_position = (viewport_position.as_ivec2() + window_bounds.min).neg_y();

            // accomodate window decorations
            let mut offset = get_window_inner_offset().neg_y();
            offset.x *= 2;
            destination_position += offset;
        }
    }

    match set_cursor_position(destination_position.neg_y()) {
        Ok(_) => {}
        Err(e) => warn!("Failed to set cursor position: {}", e),
    }
}
