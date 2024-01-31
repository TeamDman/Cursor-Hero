use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_bevy::NegativeYIVec2;
use cursor_hero_environment::environment_plugin::GameEnvironment;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_pointer_types::prelude::*;

use itertools::Itertools;

use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_toolbelt_types::prelude::*;
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
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Default { toolbelt_id }
        | PopulateToolbeltEvent::Inspector { toolbelt_id }
        | PopulateToolbeltEvent::Keyboard { toolbelt_id } = event
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
    pointers: Query<(Ref<GlobalTransform>, Option<&PointerEnvironment>), With<Pointer>>,
    tools: Query<(Option<&ActiveTool>, &Parent), With<CursorTool>>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<(&RawHandleWrapper, &Window), With<PrimaryWindow>>,
    environment_query: Query<(), With<GameEnvironment>>,
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
    let pointer = character_children
        .iter()
        .filter_map(|x| pointers.get(*x).ok())
        .next()
        .expect("Character should have a pointer");
    let (pointer_position, pointer_environment) = pointer;
    // ensure a change has occurred
    if !pointer_position.is_changed() && !character_position.is_changed() {
        // debug!("No change in pointer position or character position");
        return;
    }

    // get the destination position
    let mut destination_position = pointer_position.translation().xy().as_ivec2();

    // only when focused, do repositioning logic for when the cursor is over the window
    let (window_handle, _window) = window_query.get_single().expect("Need a single window");

    let is_in_game_environment = pointer_environment
        .and_then(|pointer_environment| {
            environment_query
                .get(pointer_environment.environment_id)
                .ok()
        })
        .is_some();
    if is_in_game_environment {
        // position the cursor over the pointer instead
        let window_bounds = match window_handle.window_handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_window_bounds(handle.hwnd as _).expect("Need a window position")
            }
            _ => panic!("Unsupported window handle"),
        };

        let camera = camera_query.get_single().expect("Need a single camera");
        let (camera_transform, camera) = camera;
        let viewport_position =
            camera.world_to_viewport(camera_transform, destination_position.as_vec2().extend(0.0));

        // if the pointer is in view, 
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
