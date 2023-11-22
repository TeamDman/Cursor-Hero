use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera_plugin::MainCamera;
use crate::character_plugin::Character;

pub struct ClickDragMovementPlugin;

impl Plugin for ClickDragMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_drag)
            .insert_resource(MouseDragState { is_dragging: false });
    }
}

#[derive(Resource)]
struct MouseDragState {
    is_dragging: bool,
}

fn update_drag(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_drag_state: ResMut<MouseDragState>,
    mut character: Query<&mut Transform, With<Character>>,
    camera_query: Query<&mut Transform, (With<MainCamera>, Without<Character>)>,
) {
    for event in mouse_button_input_events.read() {
        match event.button {
            MouseButton::Left => {
                mouse_drag_state.is_dragging = event.state.is_pressed();
            }
            _ => {}
        }
    }

    if mouse_drag_state.is_dragging {
        for event in mouse_motion_events.read() {
            let mut delta = event.delta.extend(0.0) * camera_query.single().scale;
            delta.x *= -1.0;
            for mut transform in character.iter_mut() {
                transform.translation += delta;
            }
        }
    }
}
