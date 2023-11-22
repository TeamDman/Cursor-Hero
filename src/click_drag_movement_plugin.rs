use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera_plugin::MainCamera;
use crate::character_plugin::Character;
use crate::update_ordering::MovementSet;

pub struct ClickDragMovementPlugin;

impl Plugin for ClickDragMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_drag.in_set(MovementSet::Input))
            .insert_resource(MouseDragState {
                drag_start_screen_position: None,
                is_dragging: false,
            });
    }
}

#[derive(Resource)]
struct MouseDragState {
    drag_start_screen_position: Option<Vec2>,
    is_dragging: bool,
}

fn update_drag(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_drag_state: ResMut<MouseDragState>,
    mut character: Query<&mut Transform, With<Character>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<MainCamera>, Without<Character>)>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    for event in mouse_button_input_events.read() {
        match event.button {
            MouseButton::Left => {
                mouse_drag_state.is_dragging = event.state.is_pressed();
                if mouse_drag_state.is_dragging {
                    mouse_drag_state.drag_start_screen_position = window.cursor_position();
                } else {
                    mouse_drag_state.drag_start_screen_position = None;
                }
            }
            _ => {}
        }
    }

    if mouse_drag_state.is_dragging {
        if let Some(initial_screen_position) = mouse_drag_state.drag_start_screen_position {
            if let Some(current_screen_position) = window.cursor_position() {
                // we want the starting point to remain under the mouse as we drag
                // we can calculate the world position of the starting point
                // we can calculate the world position of the current point
                // we can calculate the delta between the two
                // we can apply the delta to the character
                if let Some(initial_world_position) = camera
                    .viewport_to_world(camera_transform, initial_screen_position)
                    .map(|ray| ray.origin.truncate())
                {
                    if let Some(current_world_position) = camera
                        .viewport_to_world(camera_transform, current_screen_position)
                        .map(|ray| ray.origin.truncate())
                    {
                        let mut delta = initial_world_position - current_world_position;
                        for mut transform in character.iter_mut() {
                            transform.translation += delta.extend(0.0);
                        }
                        mouse_drag_state.drag_start_screen_position = Some(current_screen_position);
                    }
                }
            }

            // if let Some(world_position) = window
            //     .cursor_position()
            //     .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            //     .map(|ray| ray.origin.truncate())
            // {
            //     let delta = world_position - initial_screen_position;
            //     for mut transform in character.iter_mut() {
            //         transform.translation += delta.extend(0.0);
            //     }
            //     mouse_drag_state.drag_start_screen_position = Some(world_position);
            // }
        }
    }
}
