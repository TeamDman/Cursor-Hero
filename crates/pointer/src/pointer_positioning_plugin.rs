use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_bevy::NegativeYIVec2;
use cursor_hero_bevy::NegativeYVec2;
use cursor_hero_camera::camera_plugin::CameraSystemSet;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_character_types::prelude::*;
use cursor_hero_environment::environment_plugin::GameEnvironment;
use cursor_hero_environment::environment_plugin::HostEnvironment;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_pointer_types::pointer_behaviour_types::PointerMovementBehaviour;
use cursor_hero_pointer_types::prelude::*;
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

#[allow(clippy::type_complexity)]
fn update_pointer(
    mut pointer_query: Query<
        (
            &mut Transform,
            &mut Position,
            &ActionState<PointerAction>,
            &mut Pointer,
            Option<&PointerEnvironment>,
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
    input_method: Res<ActiveInput>,
    environment_query: Query<(), With<HostEnvironment>>,
    mut last_known_cursor_position: Local<Option<Vec2>>,
    mut last_sent: Local<(Option<Vec2>, Option<Vec2>, Option<IVec2>)>,
) {
    for pointer in pointer_query.iter_mut() {
        // get pointer
        let (
            mut pointer_transform,
            mut pointer_position,
            pointer_actions,
            mut pointer,
            pointer_environment,
            pointer_parent,
        ) = pointer;
        let stick = pointer_actions.pressed(PointerAction::Move);
        let in_host_environment = match pointer_environment {
            Some(pointer_environment) => {
                match environment_query.get(pointer_environment.environment_id) {
                    Ok(_) => true,
                    Err(_) => {
                        // not found because the current environment doesn't exist or is not a host environment
                        false
                    }
                }
            }
            None => false,
        };

        // get character
        let Ok(character) = character_query.get_mut(pointer_parent.get()) else {
            warn!("No character found");
            continue;
        };
        let (character_global_transform, is_main_character) = character;

        // get camera
        let Ok(camera) = camera_query.get_single() else {
            warn!("No camera found");
            return;
        };
        let (camera, camera_global_transform) = camera;

        // get window
        let Ok(window) = window_query.get_single() else {
            warn!("No window found");
            return;
        };
        let (window, window_handle) = window;

        // determine movement behaviour
        let next_behaviour = match (
            pointer.movement_behaviour,
            is_main_character.is_some(),
            in_host_environment,
            stick,
            *input_method,
        ) {
            // main character, in any environment, stick in use, mouse and keyboard
            // (_, true, _, true, ActiveInput::MouseAndKeyboard) => {
            //     PointerMovementBehaviour::HostFollowsPointer
            // }
            // main character, in host environment, stick in use
            (_, true, true, true, _) => PointerMovementBehaviour::HostFollowsPointer,
            // main character, in host environment, stick not in use, gamepad
            (_, true, true, false, ActiveInput::Gamepad) => PointerMovementBehaviour::HostFollowsPointer,
            // main character, in any environment, stick not in use
            (_, true, _, false, ActiveInput::MouseAndKeyboard) => {
                PointerMovementBehaviour::PointerFollowsHost
            }
            // main character, not in host environment, stick not in use
            (current, true, false, false, _) => current,
            // main character, not in host environment, stick in use
            (_, true, false, true, _) => PointerMovementBehaviour::HostOverWindow,
            (current, a, b, c, d) => {
                warn!("Unhandled case: current={} main_character={} in_host_environment={} stick={} input_method={:?}", current, a, b, c, d);
                current
            }
        };
        if next_behaviour != pointer.movement_behaviour {
            info!(
                "Switching to {:?} | current={} main_character={} in_host_environment={} stick={} input_method={:?}",
                next_behaviour, 
                pointer.movement_behaviour,
                is_main_character.is_some(),
                in_host_environment,
                stick,
                *input_method
            );
            pointer.movement_behaviour = next_behaviour;
        }

        let (local_target, global_target, host_target) = match pointer.movement_behaviour {
            PointerMovementBehaviour::PointerFollowsHost => {
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

                        (Some(local_target), Some(global_target), None)
                    }
                    None => {
                        if pointer.log_behaviour == PointerLogBehaviour::ErrorsAndPositionUpdates {
                            warn!("No cursor position found");
                        }
                        (None, None, None)
                    }
                }
            }
            PointerMovementBehaviour::HostFollowsPointer => {
                // host follows pointer, render and physics are the same
                if stick {
                    match pointer_actions.axis_pair(PointerAction::Move) {
                        Some(axis_pair) => {
                            let look = axis_pair.xy();
                            if look.x.is_nan() || look.y.is_nan() {
                                warn!("{} | look vector is unusable", pointer.movement_behaviour);
                                (None, None, None)
                            } else {
                                let character_translation =
                                    character_global_transform.translation();
                                let local_target = look * pointer.reach;
                                let global_target = character_translation.xy() + local_target;
                                let host_target = global_target.neg_y().as_ivec2();
                                (Some(local_target), Some(global_target), Some(host_target))
                            }
                        }
                        None => {
                            warn!("{}, No axis pair found?", pointer.movement_behaviour);
                            (None, None, None)
                        }
                    }
                } else {
                    // pointer stick not in use, reset pointer to the origin of the character
                    let character_translation = character_global_transform.translation();
                    let local_target = Vec2::ZERO;
                    let global_target = character_translation.xy();
                    let host_target = character_translation.xy().neg_y().as_ivec2();
                    (Some(local_target), Some(global_target), Some(host_target))
                }
            }
            PointerMovementBehaviour::HostOverWindow => {
                if stick {
                    // stick in use
                    // the host cursor will go over the pointer's window position
                    match pointer_actions.axis_pair(PointerAction::Move) {
                        Some(axis_pair) => {
                            let look = axis_pair.xy();

                            // the look vector could be unusable
                            if look.x.is_nan() || look.y.is_nan() {
                                warn!("{} | look vector is unusable", pointer.movement_behaviour);
                                (None, None, None)
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

                                (Some(local_target), Some(global_target.xy()), host_target)
                            }
                        }
                        None => {
                            warn!("{} | No axis pair found?", pointer.movement_behaviour);
                            (None, None, None)
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
                            debug!("host_target={:?} offset={:?}", host_target, offset);
                            host_target += offset;
                            host_target
                        });

                    (Some(local_target), Some(global_target), host_target)
                }
            }
            PointerMovementBehaviour::None => {
                // no movement behaviour, no render or physics
                continue;
            }
        };

        // Update positions
        if (local_target != (*last_sent).0 || global_target != (*last_sent).1)
            && let Some(local_target) = local_target
            && let Some(global_target) = global_target
        {
            let target_distance = local_target - pointer_transform.translation.xy();
            if target_distance == Vec2::ZERO {
                // Already at destination, keep physics in sync with unchanged render body
                if pointer.log_behaviour == PointerLogBehaviour::ErrorsAndPositionUpdates {
                    debug!(
                        "{} stick={:?} | keeping physics in sync, global_target={:?}",
                        pointer.movement_behaviour, stick, global_target
                    );
                }
                let pointer_position = pointer_position.bypass_change_detection();
                pointer_position.x = global_target.x;
                pointer_position.y = global_target.y;
            } else {
                // Not at destination, update render body (which physics will follow)
                if pointer.log_behaviour == PointerLogBehaviour::ErrorsAndPositionUpdates {
                    debug!(
                        "{} stick={:?} | target_distance={:?}, updating render body to local_target={:?}",
                        pointer.movement_behaviour, stick, target_distance, local_target
                    );
                }
                pointer_transform.translation.x = local_target.x;
                pointer_transform.translation.y = local_target.y;
            }
        }

        if host_target != last_sent.2
            && let Some(host_target) = host_target
        {
            match set_cursor_position(host_target) {
                Ok(_) => {
                    if pointer.log_behaviour == PointerLogBehaviour::ErrorsAndPositionUpdates {
                        debug!("{} | set host cursor to {:?}", pointer.movement_behaviour, host_target);
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

        *last_sent = (local_target, global_target, host_target);
    }
}
