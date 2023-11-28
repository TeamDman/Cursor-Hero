use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera_plugin::{update_camera_zoom, FollowWithCamera, MainCamera};
use crate::character_plugin::Character;

pub struct ClickDragMovementPlugin;

impl Plugin for ClickDragMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                mouse_drag_update
                    .after(update_camera_zoom),
                teleport_character_to_camera
                    .after(mouse_drag_update)
                    .run_if(should_teleport_character_to_camera),
            ),
        )
        .insert_resource(MouseDragState::default())
        .register_type::<MouseDragState>();
    }
}

#[derive(Reflect)]
struct Anchor {
    drag_start_world_position: Vec2,
}

#[derive(Resource, Reflect)]
struct MouseDragState {
    anchor: Option<Anchor>,
    is_dragging: bool,
}

impl Default for MouseDragState {
    fn default() -> Self {
        Self {
            anchor: None,
            is_dragging: false,
        }
    }
}

fn should_teleport_character_to_camera(
    query: Query<&FollowWithCamera, Added<FollowWithCamera>>,
    mouse_drag_state: Res<MouseDragState>,
) -> bool {
    query.iter().next().is_some() && mouse_drag_state.is_dragging
}

/// when the camera starts following a character while dragging, teleport the character to the camera
fn teleport_character_to_camera(
    mut character: Query<&mut Transform, (With<Character>, Without<MainCamera>)>,
    camera_transform_query: Query<&Transform, (With<MainCamera>, Without<Character>)>,
) {
    character.single_mut().translation = camera_transform_query.single().translation;
}

fn mouse_drag_update(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_drag_state: ResMut<MouseDragState>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut follow: Query<&mut Transform, (With<FollowWithCamera>, Without<MainCamera>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<
        (&Camera, &GlobalTransform),
        (With<MainCamera>, Without<Character>),
    >,
    mut camera_transform_query: Query<&mut Transform, (With<MainCamera>, Without<Character>)>,
) {
    let (camera, camera_global_transform) = camera_query.single();
    let window = window_query.single();

    // drag start and end logic
    for event in mouse_button_input_events.read() {
        match event.button {
            MouseButton::Left => {
                mouse_drag_state.is_dragging = event.state.is_pressed();
                if mouse_drag_state.is_dragging {
                    // begin dragging
                    if let Some(screen_position) = window.cursor_position() {
                        if let Some(world_position) = camera
                            .viewport_to_world(camera_global_transform, screen_position)
                            .map(|ray| ray.origin.truncate())
                        {
                            mouse_drag_state.anchor = Some(Anchor {
                                drag_start_world_position: world_position,
                            });
                        }
                    }
                } else {
                    // finish dragging
                    mouse_drag_state.anchor = None;
                }
            }
            _ => {}
        }
    }

    if mouse_drag_state.is_dragging {
        // perform drag update
        if let Some(anchor) = &mouse_drag_state.anchor {
            if let Some(current_screen_position) = window.cursor_position() {
                // mouse is inside the window, convert to world coords
                if let Some(current_world_position) = camera
                    .viewport_to_world(camera_global_transform, current_screen_position)
                    .map(|ray| ray.origin.truncate())
                {
                    // calculate delta from the anchor
                    let delta = anchor.drag_start_world_position - current_world_position;
                    if let Ok(mut follow) = follow.get_single_mut() {
                        // reposition the thing the camera is following
                        follow.translation += delta.extend(0.0);
                    } else {
                        // move the camera when not following something
                        camera_transform_query.single_mut().translation += delta.extend(0.0);
                    }
                    // track info needed to keep alignment with starting point
                    mouse_drag_state.anchor = Some(Anchor {
                        drag_start_world_position: anchor.drag_start_world_position,
                    });
                }
            } else {
                // cursor is outside the window, use delta to approximate mouse position
                let mut delta = mouse_motion_events
                    .read()
                    .fold(Vec2::ZERO, |acc, event| acc + event.delta);
                delta.x *= -1.0;
                if let Ok(mut follow) = follow.get_single_mut() {
                    // reposition the thing the camera is following
                    follow.translation += delta.extend(0.0);
                } else {
                    // move the camera when not following something
                    camera_transform_query.single_mut().translation += delta.extend(0.0);
                }
            }
        }
    }
}
