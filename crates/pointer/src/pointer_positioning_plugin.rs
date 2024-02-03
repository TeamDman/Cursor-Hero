use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_camera::camera_plugin::CameraSystemSet;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_character::character_plugin::MainCharacter;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_pointer_types::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PointerPositioningPlugin;
impl Plugin for PointerPositioningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            add_follow_host_cursor_tag_to_new_main_character_pointers
                .run_if(in_state(ActiveInput::MouseKeyboard)),
        );
        app.add_systems(
            Update,
            add_host_cursor_follows_tag_to_new_main_character_pointers
                .run_if(in_state(ActiveInput::Gamepad)),
        );
        app.add_systems(
            OnEnter(ActiveInput::MouseKeyboard),
            add_follow_host_cursor_tag_to_main_character_pointer,
        );
        app.add_systems(
            OnExit(ActiveInput::MouseKeyboard),
            remove_follow_host_cursor_tag_from_main_character_pointer,
        );
        app.add_systems(
            OnEnter(ActiveInput::Gamepad),
            add_host_cursor_follows_tag_to_main_character_pointer,
        );
        app.add_systems(
            OnExit(ActiveInput::Gamepad),
            remove_host_cursor_follows_tag_from_main_character_pointer,
        );
        app.add_systems(
            PostUpdate,
            update_pointer_position
                .in_set(PointerSystemSet::Position)
                .after(PhysicsSet::Sync)
                .after(TransformSystem::TransformPropagate),
        );

        app.add_systems(
            PostUpdate,
            update_pointer_from_mouse
                .in_set(PointerSystemSet::Position)
                .after(CameraSystemSet::Follow)
                .after(PhysicsSet::Sync)
                .after(TransformSystem::TransformPropagate),
        );
    }
}

// BEGIN NEW POINTER MANAGEMENT
#[allow(clippy::type_complexity)]
fn add_follow_host_cursor_tag_to_new_main_character_pointers(
    mut commands: Commands,
    pointer_query: Query<(Entity, &Parent), (Added<Pointer>, Without<FollowHostCursor>)>,
    character_query: Query<(), With<MainCharacter>>,
) {
    for pointer in pointer_query.iter() {
        let (pointer_id, pointer_parent) = pointer;
        if character_query.get(pointer_parent.get()).is_ok() {
            debug!("Adding FollowHostCursor to new pointer {:?}", pointer_id);
            commands.entity(pointer_id).insert(FollowHostCursor);
        }
    }
}

#[allow(clippy::type_complexity)]
fn add_host_cursor_follows_tag_to_new_main_character_pointers(
    mut commands: Commands,
    pointer_query: Query<(Entity, &Parent), (Added<Pointer>, Without<HostCursorFollows>)>,
    character_query: Query<(), With<MainCharacter>>,
) {
    for pointer in pointer_query.iter() {
        let (pointer_id, pointer_parent) = pointer;
        if character_query.get(pointer_parent.get()).is_ok() {
            debug!("Adding HostCursorFollows to new pointer {:?}", pointer_id);
            commands.entity(pointer_id).insert(HostCursorFollows);
        }
    }
}
// END NEW POINTER MANAGEMENT

// BEGIN FOLLOW HOST CURSOR
fn add_follow_host_cursor_tag_to_main_character_pointer(
    mut commands: Commands,
    character_query: Query<&Children, With<MainCharacter>>,
    pointer_query: Query<Entity, (With<Pointer>, Without<FollowHostCursor>)>,
) {
    for character in character_query.iter() {
        let character_children = character;
        for child_id in character_children.iter() {
            if let Ok(pointer_entity) = pointer_query.get(*child_id) {
                debug!("Adding FollowHostCursor to pointer {:?}", pointer_entity);
                commands.entity(pointer_entity).insert(FollowHostCursor);
            }
        }
    }
}

fn remove_follow_host_cursor_tag_from_main_character_pointer(
    mut commands: Commands,
    character_query: Query<&Children, With<MainCharacter>>,
    pointer_query: Query<Entity, (With<Pointer>, With<FollowHostCursor>)>,
) {
    for character in character_query.iter() {
        let character_children = character;
        for child_id in character_children.iter() {
            if let Ok(pointer_entity) = pointer_query.get(*child_id) {
                debug!(
                    "Removing FollowHostCursor from pointer {:?}",
                    pointer_entity
                );
                commands.entity(pointer_entity).remove::<FollowHostCursor>();
            }
        }
    }
}
// END FOLLOW HOST CURSOR

// BEGIN HOST CURSOR FOLLOWS
fn add_host_cursor_follows_tag_to_main_character_pointer(
    mut commands: Commands,
    pointer_query: Query<Entity, (With<Pointer>, Without<HostCursorFollows>)>,
    character_query: Query<&Children, With<MainCharacter>>,
) {
    for character in character_query.iter() {
        let character_children = character;
        for child_id in character_children.iter() {
            if let Ok(pointer_entity) = pointer_query.get(*child_id) {
                debug!("Adding HostCursorFollows to pointer {:?}", pointer_entity);
                commands.entity(pointer_entity).insert(HostCursorFollows);
            }
        }
    }
}
fn remove_host_cursor_follows_tag_from_main_character_pointer(
    mut commands: Commands,
    pointer_query: Query<Entity, (With<Pointer>, With<HostCursorFollows>)>,
    character_query: Query<&Children, With<MainCharacter>>,
) {
    for character in character_query.iter() {
        let character_children = character;
        for child_id in character_children.iter() {
            if let Ok(pointer_entity) = pointer_query.get(*child_id) {
                debug!(
                    "Removing HostCursorFollows from pointer {:?}",
                    pointer_entity
                );
                commands
                    .entity(pointer_entity)
                    .remove::<HostCursorFollows>();
            }
        }
    }
}
// END HOST CURSOR FOLLOWS
#[allow(clippy::type_complexity)]
fn update_pointer_position(
    mut pointer_query: Query<
        (
            &mut Position,
            &mut Transform,
            &ActionState<PointerAction>,
            &Pointer,
            &Parent,
        ),
        (Without<Character>, With<Pointer>, Without<FollowHostCursor>),
    >,
    mut character_query: Query<Ref<Position>, (With<Character>, Without<Pointer>)>,
    mut debounce: Local<bool>,
) {
    for pointer in pointer_query.iter_mut() {
        let (mut pointer_position, mut pointer_transform, pointer_actions, pointer, pointer_parent) =
            pointer;
        let character_position = character_query.get_mut(pointer_parent.get()).unwrap();
        if pointer_actions.pressed(PointerAction::Move) {
            let look = pointer_actions.axis_pair(PointerAction::Move).unwrap().xy();
            if look.x.is_nan() || look.y.is_nan() {
                continue;
            }

            let offset = look * pointer.reach;
            let desired_position = character_position.xy() + offset;
            let pointer_position = pointer_position.bypass_change_detection();
            pointer_position.x = desired_position.x;
            pointer_position.y = desired_position.y;
            pointer_transform.translation.x = offset.x;
            pointer_transform.translation.y = offset.y;
            *debounce = false;
            // debug!("pointer_position: {:?}", pointer_position.xy());
        } else if !*debounce || character_position.is_changed() {
            // debug!("character_position: {:?}", character_position.xy());
            let desired_position = character_position.xy();
            let pointer_position = pointer_position.bypass_change_detection();
            pointer_position.x = desired_position.x;
            pointer_position.y = desired_position.y;
            pointer_transform.translation.x = 0.0;
            pointer_transform.translation.y = 0.0;
            *debounce = true;
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_pointer_from_mouse(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<
        (&Camera, &GlobalTransform),
        (With<MainCamera>, Without<Character>, Without<Pointer>),
    >,
    character_query: Query<
        &GlobalTransform,
        (With<MainCharacter>, Without<MainCamera>, Without<Pointer>),
    >,
    mut pointer_query: Query<
        (&mut Transform, &mut Position),
        (
            With<Pointer>,
            With<FollowHostCursor>,
            Without<Character>,
            Without<MainCamera>,
        ),
    >,
    mut last_known_cursor_position: Local<Option<Vec2>>,
) {
    let Ok(camera) = camera_query.get_single() else {
        warn!("No camera found");
        return;
    };
    let (camera, camera_global_transform) = camera;

    let Ok(window) = window_query.get_single() else {
        warn!("No window found");
        return;
    };

    let Some(cursor_screen_position) = window.cursor_position().or(*last_known_cursor_position)
    else {
        // warn!("No cursor position?");
        return;
    };
    // for some reason, window.cursor_position starts returning None when not moving the mouse
    // this causes problems when the character moves and the pointer should follow
    // so let's just track it to fill in the gaps
    *last_known_cursor_position = Some(cursor_screen_position);
    // debug!("current_screen_position: {:?}", current_screen_position);

    let Some(cursor_world_position) = camera
        .viewport_to_world(camera_global_transform, cursor_screen_position)
        .map(|ray| ray.origin.truncate())
    else {
        return;
    };

    for pointer in pointer_query.iter_mut() {
        let (mut pointer_transform, mut pointer_position) = pointer;

        let Ok(character) = character_query.get_single() else {
            return;
        };
        let character_transform = character;

        let new_position = cursor_world_position;
        let new_translation = cursor_world_position - character_transform.translation().xy();
        let diff = new_translation - pointer_transform.translation.xy();
        if diff == Vec2::ZERO {
            let pointer_position = pointer_position.bypass_change_detection();
            pointer_position.x = new_position.x;
            pointer_position.y = new_position.y;
        } else {
            pointer_transform.translation.x = new_translation.x;
            pointer_transform.translation.y = new_translation.y;
        }
    }
}
