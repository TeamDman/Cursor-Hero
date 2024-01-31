use cursor_hero_pointer_types::prelude::*;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_character::character_plugin::MainCharacter;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use leafwing_input_manager::prelude::*;
use cursor_hero_character::character_plugin::Character;

pub struct PointerPositioningPlugin;
impl Plugin for PointerPositioningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_pointer_from_mouse.run_if(in_state(ActiveInput::MouseKeyboard)),
        );
        app.add_systems(
            PostUpdate,
            update_pointer_position
                .in_set(PointerSystemSet::Position)
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

fn update_pointer_position(
    mut pointer_query: Query<
        (
            &mut Position,
            &ActionState<PointerAction>,
            &Pointer,
            &Parent,
        ),
        (Without<Character>, With<Pointer>),
    >,
    mut character_query: Query<Ref<Position>, (With<Character>, Without<Pointer>)>,
    mut debounce: Local<bool>,
) {
    for pointer in pointer_query.iter_mut() {
        let (mut pointer_position, pointer_actions, pointer, pointer_parent) = pointer;
        let character_position = character_query.get_mut(pointer_parent.get()).unwrap();
        if pointer_actions.pressed(PointerAction::Move) {
            let look = pointer_actions.axis_pair(PointerAction::Move).unwrap().xy();
            if look.x.is_nan() || look.y.is_nan() {
                continue;
            }

            let offset = look * pointer.reach;
            let desired_position = character_position.xy() + offset;
            pointer_position.x = desired_position.x;
            pointer_position.y = desired_position.y;
            *debounce = false;
        } else if !*debounce || character_position.is_changed() {
            let desired_position = character_position.xy();
            pointer_position.x = desired_position.x;
            pointer_position.y = desired_position.y;
            *debounce = true;
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_pointer_from_mouse(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<MainCamera>, Without<Character>)>,
    character_query: Query<&Children, (With<MainCharacter>, Without<MainCamera>, Without<Pointer>)>,
    mut pointer_query: Query<&mut Position, With<Pointer>>,
    mut last_known_cursor_position: Local<Option<Vec2>>,
) {
    let (camera, camera_global_transform) = camera_query.single();
    let window = window_query.single();
    if let Some(current_screen_position) = window.cursor_position().or(*last_known_cursor_position)
    {
        // for some reason, window.cursor_position starts returning None when not moving the mouse
        // this causes problems when the character moves and the pointer should follow
        // so let's just track it to fill in the gaps
        *last_known_cursor_position = Some(current_screen_position);
        // mouse is inside the window, convert to world coords
        if let Some(current_world_position) = camera
            .viewport_to_world(camera_global_transform, current_screen_position)
            .map(|ray| ray.origin.truncate())
        {
            if let Ok(character_children) = character_query.get_single() {
                for child in character_children.iter() {
                    if let Ok(mut pointer_position) = pointer_query.get_mut(*child) {
                        pointer_position.x = current_world_position.x;
                        pointer_position.y = current_world_position.y;
                    }
                }
            }
        }
    }
}
