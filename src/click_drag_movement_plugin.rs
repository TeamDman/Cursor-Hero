use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera_plugin::{camera_zoom_tick, MainCamera};
use crate::character_plugin::Character;
use crate::update_ordering::MovementSet;

pub struct ClickDragMovementPlugin;

impl Plugin for ClickDragMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_drag
                .in_set(MovementSet::Input)
                .after(camera_zoom_tick),
        )
        .insert_resource(MouseDragState::default());
    }
}

struct StartState {
    drag_start_screen_position: Vec2,
    drag_start_world_position: Vec2,
}

#[derive(Resource)]
struct MouseDragState {
    start_state: Option<StartState>,
    is_dragging: bool,
    last_known_screen_position: Option<Vec2>,
}

impl Default for MouseDragState {
    fn default() -> Self {
        Self {
            start_state: None,
            is_dragging: false,
            last_known_screen_position: None,
        }
    }
}

fn update_drag(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_drag_state: ResMut<MouseDragState>,
    mut mouse_motion_events: EventReader<MouseMotion>,
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
                    // begin dragging
                    if let Some(screen_position) = window.cursor_position() {
                        if let Some(world_position) = camera
                            .viewport_to_world(camera_transform, screen_position)
                            .map(|ray| ray.origin.truncate())
                        {
                            mouse_drag_state.start_state = Some(StartState {
                                drag_start_screen_position: screen_position,
                                drag_start_world_position: world_position,
                            });
                            mouse_drag_state.last_known_screen_position = Some(screen_position);
                        }
                    }
                } else {
                    // finish dragging
                    mouse_drag_state.start_state = None;
                    mouse_drag_state.last_known_screen_position = None;
                }
            }
            _ => {}
        }
    }

    if mouse_drag_state.is_dragging {
        // perform drag update
        if let Some(start_state) = &mouse_drag_state.start_state {
            if let Some(current_screen_position) = window.cursor_position() {
                // we want the starting point to remain under the mouse as we drag
                if let Some(current_world_position) = camera
                    .viewport_to_world(camera_transform, current_screen_position)
                    .map(|ray| ray.origin.truncate())
                {
                    let delta = start_state.drag_start_world_position - current_world_position;
                    for mut transform in character.iter_mut() {
                        transform.translation += delta.extend(0.0);
                    }
                    mouse_drag_state.start_state = Some(StartState {
                        drag_start_screen_position: current_screen_position,
                        drag_start_world_position: start_state.drag_start_world_position,
                    });
                    mouse_drag_state.last_known_screen_position = Some(current_screen_position);
                }
            } else {
                // cursor is outside the window, use delta to approximate mouse position
                let mut delta = mouse_motion_events.read().fold(Vec2::ZERO, |acc, event| {
                    acc + event.delta
                });
                delta.x *= -1.0;
                for mut transform in character.iter_mut() {
                    transform.translation += delta.extend(0.0);
                }
            }
        }
    }
}
