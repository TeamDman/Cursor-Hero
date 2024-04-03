use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_bevy::prelude::NegativeYVec2;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_character_types::prelude::*;
use cursor_hero_environment_types::prelude::*;
use cursor_hero_input::active_input_state_plugin::InputMethod;
use cursor_hero_cursor_types::pointer_behaviour_types::PointerMovementBehaviour;
use cursor_hero_cursor_types::prelude::*;
use cursor_hero_winutils::win_mouse::set_cursor_position;
use cursor_hero_winutils::win_window::get_window_bounds;
use cursor_hero_winutils::win_window::get_window_inner_offset;
use leafwing_input_manager::prelude::*;

use bevy::window::RawHandleWrapper;

pub struct PointerPositioningPlugin;
impl Plugin for PointerPositioningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (update_pointer)
                .in_set(PointerSystemSet::Position)
                .after(PhysicsSet::Sync)
                .after(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(Default, Debug)]
struct PointerUpdate {
    local_target: Option<Vec2>,
    global_target: Option<Vec2>,
    host_target: Option<IVec2>,
}

#[derive(Debug)]
struct DecisionInfo {
    current_behaviour: PointerMovementBehaviour,
    is_main_character: bool,
    in_host_environment: bool,
    stick_in_use: bool,
    active_input_method: InputMethod,
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn update_pointer(
    mut pointer_query: Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut Position,
            &ActionState<PointerAction>,
            &mut Pointer,
            Option<&EnvironmentTracker>,
            &Parent,
        ),
        (Without<Character>, With<Pointer>),
    >,
    mut character_query: Query<
        (Ref<GlobalTransform>, Option<&MainCharacter>),
        (With<Character>, Without<Pointer>, Without<MainCamera>),
    >,
    camera_query: Query<
        (&Camera, &GlobalTransform),
        (With<MainCamera>, Without<Character>, Without<Pointer>),
    >,
    window_query: Query<(&Window, &RawHandleWrapper), With<PrimaryWindow>>,
    input_method: Res<InputMethod>,
    environment_query: Query<(), With<HostEnvironment>>,
    mut last_known_cursor_position: Local<Option<Vec2>>,
    mut previous_update: Local<PointerUpdate>,
) {
    for pointer in pointer_query.iter_mut() {
        let (
            mut pointer_transform,
            pointer_global_transform,
            mut pointer_position,
            pointer_actions,
            mut pointer,
            pointer_environment,
            pointer_parent,
        ) = pointer;

        let stick_in_use = pointer_actions.pressed(PointerAction::Move);
        let in_host_environment = pointer_environment
            .map(|e| environment_query.contains(e.environment_id))
            .unwrap_or(false);

        let Ok(character) = character_query.get_mut(pointer_parent.get()) else {
            warn!("No character found");
            continue;
        };
        let (character_global_transform, is_main_character) = character;

        let Ok(camera) = camera_query.get_single() else {
            warn!("No camera found");
            return;
        };
        let (camera, camera_global_transform) = camera;

        let Ok(window) = window_query.get_single() else {
            warn!("No window found");
            return;
        };
        let (window, window_handle) = window;

        let decision_info = DecisionInfo {
            current_behaviour: pointer.movement_behaviour,
            is_main_character: is_main_character.is_some(),
            in_host_environment,
            stick_in_use,
            active_input_method: *input_method,
        };

        let next_behaviour = match decision_info {
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
            } => PointerMovementBehaviour::SetHostCursorFromPointerWorldCoords,
            DecisionInfo {
                is_main_character: true,
                stick_in_use: false,
                active_input_method: InputMethod::MouseAndKeyboard,
                ..
            } => PointerMovementBehaviour::SetPointerFromHostCursorWindowCoords,
            DecisionInfo {
                is_main_character: true,
                in_host_environment: false,
                stick_in_use: true,
                ..
            } => PointerMovementBehaviour::SetHostCursorFromWindowCoords,
            DecisionInfo {
                is_main_character: true,
                in_host_environment: false,
                stick_in_use: false,
                ..
            } => decision_info.current_behaviour,
            DecisionInfo {
                current_behaviour: PointerMovementBehaviour::None,
                ..
            } => PointerMovementBehaviour::None,
            _ => {
                warn!("Unhandled case: {:?}", decision_info);
                decision_info.current_behaviour
            }
        };

        if next_behaviour != pointer.movement_behaviour {
            info!(
                "Switching to {:?} given {:?}",
                next_behaviour, decision_info
            );
            pointer.movement_behaviour = next_behaviour;
        }

        let this_update = match pointer.movement_behaviour {
            PointerMovementBehaviour::None => {
                // sync physics to render
                PointerUpdate {
                    local_target: None,
                    global_target: Some(pointer_global_transform.translation().xy()),
                    host_target: None,
                }
            }
            PointerMovementBehaviour::SetPointerFromHostCursorWindowCoords => {
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
                        PointerUpdate {
                            local_target: Some(local_target),
                            global_target: Some(global_target),
                            host_target: None,
                        }
                    }
                    None => {
                        if pointer.log_behaviour == PointerLogBehaviour::ErrorsAndPositionUpdates {
                            warn!("No cursor position found");
                        }
                        PointerUpdate::default()
                    }
                }
            }
            PointerMovementBehaviour::SetHostCursorFromPointerWorldCoords => {
                // host follows pointer, render and physics are the same
                if stick_in_use {
                    match pointer_actions.axis_pair(PointerAction::Move) {
                        Some(axis_pair) => {
                            let look = axis_pair.xy();
                            if look.x.is_nan() || look.y.is_nan() {
                                warn!("{} | look vector is unusable", pointer.movement_behaviour);
                                PointerUpdate::default()
                            } else {
                                let character_translation =
                                    character_global_transform.translation();
                                let local_target = look * pointer.reach;
                                let global_target = character_translation.xy() + local_target;
                                let host_target = global_target.neg_y().as_ivec2();
                                PointerUpdate {
                                    local_target: Some(local_target),
                                    global_target: Some(global_target),
                                    host_target: Some(host_target),
                                }
                            }
                        }
                        None => {
                            warn!("{}, No axis pair found?", pointer.movement_behaviour);
                            PointerUpdate::default()
                        }
                    }
                } else {
                    // pointer stick not in use, reset pointer to the origin of the character
                    let character_translation = character_global_transform.translation();
                    let local_target = Vec2::ZERO;
                    let global_target = character_translation.xy();
                    let host_target = character_translation.xy().neg_y().as_ivec2();
                    PointerUpdate {
                        local_target: Some(local_target),
                        global_target: Some(global_target),
                        host_target: Some(host_target),
                    }
                }
            }
            PointerMovementBehaviour::SetHostCursorFromWindowCoords => {
                if stick_in_use {
                    // stick in use
                    // the host cursor will go over the pointer's window position
                    match pointer_actions.axis_pair(PointerAction::Move) {
                        Some(axis_pair) => {
                            let look = axis_pair.xy();

                            // the look vector could be unusable
                            if look.x.is_nan() || look.y.is_nan() {
                                warn!("{} | look vector is unusable", pointer.movement_behaviour);
                                PointerUpdate::default()
                            } else {
                                // the spot you want to be is the character position + stick direction
                                let character_translation =
                                    character_global_transform.translation();
                                let local_target = look * pointer.reach;
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

                                PointerUpdate {
                                    local_target: Some(local_target.xy()),
                                    global_target: Some(global_target.xy()),
                                    host_target,
                                }
                            }
                        }
                        None => {
                            warn!("{} | No axis pair found?", pointer.movement_behaviour);
                            PointerUpdate::default()
                        }
                    }
                } else {
                    // stick not in use
                    // reset pointer to the origin of the character
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

                    PointerUpdate {
                        local_target: Some(local_target),
                        global_target: Some(global_target),
                        host_target,
                    }
                }
            }
        };

        // Update render body
        let mut render_updated = false;
        if this_update.local_target != previous_update.local_target
            && let Some(local_target) = this_update.local_target
        {
            let target_distance = local_target - pointer_transform.translation.xy();
            if target_distance != Vec2::ZERO {
                // Not at destination, update render body (which physics will follow)
                if pointer.log_behaviour == PointerLogBehaviour::ErrorsAndPositionUpdates {
                    debug!(
                        "{} stick={:?} | target_distance={:?}, updating render body to local_target={:?}",
                        pointer.movement_behaviour, stick_in_use, target_distance, local_target
                    );
                }
                pointer_transform.translation.x = local_target.x;
                pointer_transform.translation.y = local_target.y;
                render_updated = true;
            }
        }

        // Update physics body
        if !render_updated
            && this_update.global_target != previous_update.global_target
            && let Some(global_target) = this_update.global_target
        {
            let target_distance = global_target - pointer_position.xy();
            if target_distance != Vec2::ZERO {
                // Not at destination, update physics body
                if pointer.log_behaviour == PointerLogBehaviour::ErrorsAndPositionUpdates {
                    debug!(
                        "{} stick={:?} | target_distance={:?}, updating physics body to global_target={:?}",
                        pointer.movement_behaviour, stick_in_use, target_distance, global_target
                    );
                }
                // prevent feedback loop
                let pointer_position = pointer_position.bypass_change_detection();

                // update physics body
                pointer_position.x = global_target.x;
                pointer_position.y = global_target.y;
            }
        }

        if this_update.host_target != previous_update.host_target
            && let Some(host_target) = this_update.host_target
        {
            match set_cursor_position(host_target) {
                Ok(_) => {
                    if pointer.log_behaviour == PointerLogBehaviour::ErrorsAndPositionUpdates {
                        debug!(
                            "{} | set host cursor to {:?}",
                            pointer.movement_behaviour, host_target
                        );
                    }
                }
                Err(e) => {
                    warn!(
                        "{} | host cursor update failed, tried setting to {:?}, error={:?}",
                        pointer.movement_behaviour, host_target, e
                    );
                }
            }
        }

        *previous_update = this_update;
    }
}
