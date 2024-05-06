use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_bevy::prelude::NegativeYVec2;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_character_types::prelude::*;
use cursor_hero_cursor_types::cursor_behaviour_types::CursorMovementBehaviour;
use cursor_hero_cursor_types::prelude::*;
use cursor_hero_environment_types::prelude::*;
use cursor_hero_input::active_input_state_plugin::InputMethod;
use cursor_hero_winutils::win_mouse::set_cursor_position;
use cursor_hero_winutils::win_window::get_window_bounds;
use cursor_hero_winutils::win_window::get_window_inner_offset;
use leafwing_input_manager::prelude::*;

use bevy::window::RawHandleWrapper;

pub struct CursorPositioningPlugin;
impl Plugin for CursorPositioningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (update_cursor)
                .in_set(CursorSystemSet::Position)
                .after(PhysicsSet::Sync)
                .after(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(Default, Debug)]
struct CursorUpdate {
    local_transform: Option<Vec2>,
    global_position: Option<Vec2>,
    host_cursor: Option<IVec2>,
}

#[derive(Debug)]
struct DecisionInfo {
    current_behaviour: CursorMovementBehaviour,
    is_main_character: bool,
    in_host_environment: bool,
    stick_in_use: bool,
    active_input_method: InputMethod,
    desired_position: Option<Vec2>,
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn update_cursor(
    mut cursor_query: Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut Position,
            &ActionState<CursorAction>,
            &mut Cursor,
            Option<&TrackedEnvironment>,
            &Parent,
        ),
        Without<Character>,
    >,
    mut character_query: Query<
        (Ref<GlobalTransform>, Option<&MainCharacter>),
        (With<Character>, Without<Cursor>, Without<MainCamera>),
    >,
    camera_query: Query<
        (&Camera, &GlobalTransform),
        (With<MainCamera>, Without<Character>, Without<Cursor>),
    >,
    window_query: Query<(&Window, &RawHandleWrapper), With<PrimaryWindow>>,
    input_method: Res<State<InputMethod>>,
    environment_query: Query<(), With<HostEnvironment>>,
    mut last_known_cursor_position: Local<Option<Vec2>>,
    mut previous_update: Local<CursorUpdate>,
) {
    // for each cursor
    for cursor in cursor_query.iter_mut() {
        let (
            mut cursor_transform,
            cursor_global_transform,
            mut cursor_position,
            cursor_actions,
            mut cursor,
            cursor_environment,
            cursor_parent,
        ) = cursor;

        // Get controller state
        let stick_in_use = cursor_actions.pressed(CursorAction::Move);

        // Get host environment presence
        let in_host_environment = cursor_environment
            .map(|e| environment_query.contains(e.environment_id))
            .unwrap_or(false);

        // Get character
        let Ok(character) = character_query.get_mut(cursor_parent.get()) else {
            warn!("No character found");
            continue;
        };
        let (character_global_transform, is_main_character) = character;

        // Get camera
        let Ok(camera) = camera_query.get_single() else {
            warn!("No camera found");
            return;
        };
        let (camera, camera_global_transform) = camera;

        // Get window
        let Ok(window) = window_query.get_single() else {
            warn!("No window found");
            return;
        };
        let (window, window_handle) = window;

        // Prepare decision info
        let decision_info = DecisionInfo {
            current_behaviour: cursor.movement_behaviour,
            is_main_character: is_main_character.is_some(),
            in_host_environment,
            stick_in_use,
            active_input_method: *input_method.get(),
            desired_position: cursor.desired_position,
        };

        // Determine behaviour
        let next_behaviour = match decision_info {
            DecisionInfo {
                desired_position: Some(ref _pos),
                ..
            } => CursorMovementBehaviour::SetBothToDesiredCoords,
            DecisionInfo {
                is_main_character: true,
                in_host_environment: true,
                stick_in_use: true,
                ..
            }
            | DecisionInfo {
                is_main_character: true,
                in_host_environment: true,
                active_input_method: InputMethod::Gamepad,
                ..
            }
            | DecisionInfo {
                is_main_character: true,
                in_host_environment: true,
                active_input_method: InputMethod::Keyboard,
                ..
            } => CursorMovementBehaviour::SetHostCursorFromCursorWorldCoords,
            DecisionInfo {
                is_main_character: true,
                stick_in_use: false,
                active_input_method: InputMethod::MouseAndKeyboard,
                ..
            } => CursorMovementBehaviour::SetCursorFromHostCursorWindowCoords,
            DecisionInfo {
                is_main_character: true,
                in_host_environment: false,
                stick_in_use: true,
                ..
            } => CursorMovementBehaviour::SetHostCursorFromWindowCoords,
            DecisionInfo {
                is_main_character: true,
                in_host_environment: false,
                stick_in_use: false,
                ..
            } => decision_info.current_behaviour,
            DecisionInfo {
                current_behaviour: CursorMovementBehaviour::None,
                ..
            } => CursorMovementBehaviour::None,
            _ => {
                warn!("Unhandled case: {:?}", decision_info);
                decision_info.current_behaviour
            }
        };

        // Update behaviour if changed
        if next_behaviour != cursor.movement_behaviour {
            // Announce change
            info!(
                "Switching to {:?} given {:?}",
                next_behaviour, decision_info
            );

            // Update behaviour
            cursor.movement_behaviour = next_behaviour;
        }

        // Get cursor update
        let this_update = match cursor.movement_behaviour {
            CursorMovementBehaviour::None => {
                // sync physics to render
                CursorUpdate {
                    local_transform: None,
                    global_position: Some(cursor_global_transform.translation().xy()),
                    host_cursor: None,
                }
            }
            CursorMovementBehaviour::SetCursorFromHostCursorWindowCoords => {
                // usual mode for mouse and keyboard input
                match window.cursor_position().or(*last_known_cursor_position) {
                    Some(host_cursor_xy) => {
                        // Cache to avoid jitter
                        *last_known_cursor_position = Some(host_cursor_xy);

                        // Calculate target positions
                        let Some(global_target) = camera
                            .viewport_to_world(camera_global_transform, host_cursor_xy)
                            .map(|ray| ray.origin.truncate())
                        else {
                            return;
                        };
                        let local_target =
                            global_target - character_global_transform.translation().xy();
                        CursorUpdate {
                            local_transform: Some(local_target),
                            global_position: Some(global_target),
                            host_cursor: None,
                        }
                    }
                    None => {
                        if cursor.log_behaviour == CursorLogBehaviour::ErrorsAndPositionUpdates {
                            warn!("No cursor position found");
                        }
                        CursorUpdate::default()
                    }
                }
            }
            CursorMovementBehaviour::SetHostCursorFromCursorWorldCoords => {
                // host follows cursor, render and physics are the same
                if stick_in_use {
                    match cursor_actions.axis_pair(CursorAction::Move) {
                        Some(axis_pair) => {
                            let look = axis_pair.xy();
                            if look.x.is_nan() || look.y.is_nan() {
                                warn!("{} | look vector is unusable", cursor.movement_behaviour);
                                CursorUpdate::default()
                            } else {
                                let character_translation =
                                    character_global_transform.translation();
                                let local_target = look * cursor.reach;
                                let global_target = character_translation.xy() + local_target;
                                let host_target = global_target.neg_y().as_ivec2();
                                CursorUpdate {
                                    local_transform: Some(local_target),
                                    global_position: Some(global_target),
                                    host_cursor: Some(host_target),
                                }
                            }
                        }
                        None => {
                            warn!("{}, No axis pair found?", cursor.movement_behaviour);
                            CursorUpdate::default()
                        }
                    }
                } else {
                    // cursor stick not in use, reset cursor to the origin of the character
                    let character_translation = character_global_transform.translation();
                    let local_target = Vec2::ZERO;
                    let global_target = character_translation.xy();
                    let host_target = character_translation.xy().neg_y().as_ivec2();
                    CursorUpdate {
                        local_transform: Some(local_target),
                        global_position: Some(global_target),
                        host_cursor: Some(host_target),
                    }
                }
            }
            CursorMovementBehaviour::SetBothToDesiredCoords => {
                // set both cursor and host cursor to desired position
                if let Some(desired_position) = cursor.desired_position {
                    let character_translation = character_global_transform.translation();
                    let local_target = desired_position - character_translation.xy();
                    let global_target = desired_position;
                    let host_target = desired_position.neg_y().as_ivec2();
                    CursorUpdate {
                        local_transform: Some(local_target),
                        global_position: Some(global_target),
                        host_cursor: Some(host_target),
                    }
                } else {
                    warn!("{} | No desired position found", cursor.movement_behaviour);
                    CursorUpdate::default()
                }
            }
            CursorMovementBehaviour::SetHostCursorFromWindowCoords => {
                if stick_in_use {
                    // stick in use
                    // the host cursor will go over the cursor's window position
                    match cursor_actions.axis_pair(CursorAction::Move) {
                        Some(axis_pair) => {
                            let look = axis_pair.xy();

                            // the look vector could be unusable
                            if look.x.is_nan() || look.y.is_nan() {
                                warn!("{} | look vector is unusable", cursor.movement_behaviour);
                                CursorUpdate::default()
                            } else {
                                // the spot you want to be is the character position + stick direction
                                let character_translation =
                                    character_global_transform.translation();
                                let local_target = look * cursor.reach;
                                let global_target =
                                    character_translation + local_target.extend(0.0);

                                // update the host cursor
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
                                let host_target = camera
                                    .world_to_viewport(camera_global_transform, global_target)
                                    .map(|viewport| {
                                        let mut host_target =
                                            viewport.as_ivec2() + window_bounds.min;
                                        // accomodate window decorations
                                        let mut offset = get_window_inner_offset();
                                        offset.x *= 2;
                                        host_target += offset;
                                        host_target
                                    });

                                CursorUpdate {
                                    local_transform: Some(local_target.xy()),
                                    global_position: Some(global_target.xy()),
                                    host_cursor: host_target,
                                }
                            }
                        }
                        None => {
                            warn!("{} | No axis pair found?", cursor.movement_behaviour);
                            CursorUpdate::default()
                        }
                    }
                } else {
                    // stick not in use
                    // reset cursor to the origin of the character
                    let character_translation = character_global_transform.translation();
                    let local_target = Vec2::ZERO;
                    let global_target = character_translation.xy().neg_y();

                    // update the host cursor
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
                    let host_target = camera
                        .world_to_viewport(camera_global_transform, character_translation)
                        .map(|viewport| {
                            let mut host_target = viewport.as_ivec2() + window_bounds.min;
                            // accomodate window decorations
                            let mut offset = get_window_inner_offset();
                            offset.x *= 2;
                            // debug!("host_target={:?} offset={:?}", host_target, offset);
                            host_target += offset;
                            host_target
                        });

                    CursorUpdate {
                        local_transform: Some(local_target),
                        global_position: Some(global_target),
                        host_cursor: host_target,
                    }
                }
            }
        };

        // Update render body
        let mut render_updated = false;
        if this_update.local_transform != previous_update.local_transform
            && let Some(local_target) = this_update.local_transform
        {
            let target_distance = local_target - cursor_transform.translation.xy();
            if target_distance != Vec2::ZERO {
                // Not at destination, update render body (which physics will follow)
                if cursor.log_behaviour == CursorLogBehaviour::ErrorsAndPositionUpdates {
                    debug!(
                        "{} stick={:?} | target_distance={:?}, updating render body to local_target={:?}",
                        cursor.movement_behaviour, stick_in_use, target_distance, local_target
                    );
                }
                cursor_transform.translation.x = local_target.x;
                cursor_transform.translation.y = local_target.y;
                render_updated = true;
            }
        }

        // Update physics body
        if !render_updated
            && this_update.global_position != previous_update.global_position
            && let Some(global_target) = this_update.global_position
        {
            let target_distance = global_target - cursor_position.xy();
            if target_distance != Vec2::ZERO {
                // Not at destination, update physics body
                if cursor.log_behaviour == CursorLogBehaviour::ErrorsAndPositionUpdates {
                    debug!(
                        "{} stick={:?} | target_distance={:?}, updating physics body to global_target={:?}",
                        cursor.movement_behaviour, stick_in_use, target_distance, global_target
                    );
                }
                // prevent feedback loop
                let cursor_position = cursor_position.bypass_change_detection();

                // update physics body
                cursor_position.x = global_target.x;
                cursor_position.y = global_target.y;
            }
        }

        if this_update.host_cursor != previous_update.host_cursor
            && let Some(host_target) = this_update.host_cursor
        {
            match set_cursor_position(host_target) {
                Ok(_) => {
                    if cursor.log_behaviour == CursorLogBehaviour::ErrorsAndPositionUpdates {
                        debug!(
                            "{} | set host cursor to {:?}",
                            cursor.movement_behaviour, host_target
                        );
                    }
                }
                Err(e) => {
                    warn!(
                        "{} | host cursor update failed, tried setting to {:?}, error={:?}",
                        cursor.movement_behaviour, host_target, e
                    );
                }
            }
        }

        *previous_update = this_update;
    }
}
