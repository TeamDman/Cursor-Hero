use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_bevy::NegativeYIVec2;
use cursor_hero_environment::environment_plugin::GameEnvironment;
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

#[allow(clippy::type_complexity)]
fn snap_mouse_to_pointer(
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<(Ref<GlobalTransform>, &Children), With<Character>>,
    pointers: Query<
        (Ref<GlobalTransform>, Option<&PointerEnvironment>),
        (With<Pointer>, With<HostCursorFollows>),
    >,
    tools: Query<&Parent, (With<CursorTool>, With<ActiveTool>)>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&RawHandleWrapper, With<PrimaryWindow>>,
    environment_query: Query<(), With<GameEnvironment>>,
) {
    let active = tools.iter().collect_vec();
    let active_count = active.len();
    if active_count > 1 {
        warn!("Only one cursor positioning tool should be active at a time");
    }
    if active_count == 0 {
        return;
    }

    let Some(tool) = active.first() else {
        return;
    };
    let tool_parent = tool;

    let Ok(toolbelt) = toolbelts.get(tool_parent.get()) else {
        warn!("Tool not inside a toolbelt?");
        return;
    };
    let toolbelt_parent = toolbelt;

    let Ok(character) = characters.get(toolbelt_parent.get()) else {
        warn!("Toolbelt parent not a character?");
        return;
    };
    let (character_position, character_children) = character;

    let Some(pointer) = character_children
        .iter()
        .filter_map(|x| pointers.get(*x).ok())
        .next()
    else {
        // may not be any pointers matching With<HostCursorFollows>
        return;
    };
    let (pointer_position, pointer_environment) = pointer;

    // ensure a change has occurred
    if !pointer_position.is_changed() && !character_position.is_changed() {
        // debug!("No change in pointer position or character position");
        return;
    }

    let mut destination_position = pointer_position.translation().xy().as_ivec2();

    let Ok(window) = window_query.get_single() else {
        error!("No primary window found");
        return;
    };
    let window_handle = window;

    let is_in_game_environment = pointer_environment
        .and_then(|pointer_environment| {
            environment_query
                .get(pointer_environment.environment_id)
                .ok()
        })
        .is_some();
    if is_in_game_environment {
        destination_position += IVec2::new(1, -1);
        // position the cursor over the pointer instead
        let window_bounds = match window_handle.window_handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                match get_window_bounds(handle.hwnd as _) {
                    Ok(bounds) => bounds,
                    Err(e) => {
                        error!("Failed to get window bounds: {:?}", e);
                        return;
                    }
                }
            }
            _ => panic!("Unsupported window handle"),
        };

        let Ok(camera) = camera_query.get_single() else {
            error!("No camera found");
            return;
        };
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

    // debug!("Setting cursor position to {:?}", destination_position);
    match set_cursor_position(destination_position.neg_y()) {
        Ok(_) => {}
        Err(e) => warn!("Failed to set cursor position: {}", e),
    }
}
