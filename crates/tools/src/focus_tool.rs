use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use cursor_hero_bevy::prelude::NegativeYVec2;
use cursor_hero_camera::camera_plugin::FollowWithMainCamera;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_cursor_types::cursor_types::Cursor;
use cursor_hero_winutils::win_mouse::set_cursor_position;
use cursor_hero_winutils::win_window::get_window_title_bar_center_position;
use leafwing_input_manager::prelude::*;

use cursor_hero_camera::camera_plugin::CameraEvent;
use cursor_hero_character_types::prelude::*;
use cursor_hero_winutils::win_window::focus_window;

use cursor_hero_toolbelt_types::prelude::*;

use crate::prelude::*;
use cursor_hero_movement_tool_types::prelude::*;

pub struct FocusToolPlugin;

impl Plugin for FocusToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FocusTool>();
        app.add_plugins(InputManagerPlugin::<FocusToolAction>::default());
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, handle_input);
    }
}

#[derive(Component, Reflect, Default, Eq, PartialEq)]
pub enum FocusMode {
    #[default]
    CameraFollowsCharacter,
    CameraWanders,
    CharacterWanders,
}
impl FocusMode {
    pub fn next(&self) -> Self {
        match self {
            Self::CameraFollowsCharacter => Self::CameraWanders,
            Self::CameraWanders => Self::CharacterWanders,
            Self::CharacterWanders => Self::CameraFollowsCharacter,
        }
    }
}

#[derive(Component, Reflect, Default)]
struct FocusTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        let ToolbeltLoadout::Default = event.loadout else {
            continue;
        };
        {
            ToolSpawnConfig::<FocusTool, FocusToolAction>::new(FocusTool, event.id, event)
                .with_src_path(file!().into())
                .guess_name(file!())
                .guess_image(file!(), &asset_server, "png")
                .with_description("Camera follows the character")
                .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum FocusToolAction {
    CycleFollowTarget,
    FocusMainWindow,
}
// TODO: add an action to focus the character without teleporting it to the camera.

impl FocusToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::CycleFollowTarget => GamepadButtonType::LeftThumb.into(),
            Self::FocusMainWindow => GamepadButtonType::RightThumb.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::CycleFollowTarget => KeyCode::Space.into(),
            Self::FocusMainWindow => KeyCode::Home.into(),
        }
    }
}
impl ToolAction for FocusToolAction {
    fn default_input_map(_event: &ToolbeltPopulateEvent) -> Option<InputMap<FocusToolAction>> {
        let mut input_map = InputMap::default();

        for variant in FocusToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn handle_input(
    mut focus_tool_query: Query<
        (&ActionState<FocusToolAction>, &Parent, &mut FocusTool),
        With<ActiveTool>,
    >,
    movement_tool_query: Query<(Entity, &MovementTool)>,
    toolbelt_query: Query<(&Parent, &Children), With<Toolbelt>>,
    mut character_query: Query<
        (
            Entity,
            &mut Transform,
            Option<&FollowWithMainCamera>,
            &Children,
        ),
        (With<Character>, Without<MainCamera>),
    >,
    mut cursor_query: Query<&mut Cursor>,
    camera_query: Query<(Entity, &Transform), (With<MainCamera>, Without<Character>)>,
    window_query: Query<&RawHandleWrapper, With<PrimaryWindow>>,
    mut camera_events: EventWriter<CameraEvent>,
    mut movement_target_events: EventWriter<MovementTargetEvent>,
) {
    // For each focus tool
    for tool in focus_tool_query.iter() {
        let (tool_actions, tool_parent, mut focus_tool) = tool;

        if tool_actions.just_pressed(FocusToolAction::CycleFollowTarget) {
            // Announce acknowledgement
            info!("Cycle follow target");

            // Get the toolbelt
            let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
                warn!("Toolbelt should have a parent");
                continue;
            };
            let (toolbelt_parent, toolbelt_children) = toolbelt;

            // Get the movement tool
            let mut movement_tool = None;
            for child in toolbelt_children.iter() {
                if let Ok(tool) = movement_tool_query.get(*child) {
                    movement_tool = Some(tool);
                    break;
                }
            }
            let Some((movement_tool_id, movement_tool)) = movement_tool else {
                warn!("Toolbelt should have a movement tool");
                continue;
            };

            // Get the character
            let Ok(character) = character_query.get_mut(toolbelt_parent.get()) else {
                warn!("Toolbelt should have a character");
                continue;
            };
            let (character_id, mut character_transform, character_is_followed, character_children) =
                character;

            // Get the camera
            let Ok(camera) = camera_query.get_single() else {
                warn!("No main camera found");
                continue;
            };
            let (camera_id, camera_transform) = camera;

            let derived_mode = match (movement_tool.target, character_is_followed) {
                (MovementTarget::Character, Some(_)) => FocusMode::CameraFollowsCharacter,
                (MovementTarget::Character, None) => FocusMode::CharacterWanders,
                (MovementTarget::Camera(_), _) => FocusMode::CameraWanders,
            };
            let next_mode = derived_mode.next();

            match next_mode {
                FocusMode::CameraFollowsCharacter => {
                    // Announce change
                    info!("Control and camera returning to character");

                    // Tell the camera to follow the character
                    camera_events.send(CameraEvent::BeginFollowing {
                        target_id: character_id,
                    });

                    // Tell the movement tools to manipulate the character
                    movement_target_events.send(MovementTargetEvent::SetTarget {
                        tool_id: movement_tool_id,
                        target: MovementTarget::Character,
                    });

                    // Snap the character to under wherever the camera has wandered
                    character_transform.translation = camera_transform.translation;
                }
                FocusMode::CameraWanders => {
                    // Announce change
                    info!("Camera now wandering from character");

                    // Tell the camera to stop following the character
                    camera_events.send(CameraEvent::StopFollowing {
                        target_id: character_id,
                    });

                    // Tell the movement tools to manipulate the camera
                    movement_target_events.send(MovementTargetEvent::SetTarget {
                        tool_id: movement_tool_id,
                        target: MovementTarget::Camera(camera_id),
                    });
                }
                FocusMode::CharacterWanders => {
                    // Announce change
                    info!("Character now wandering from camera");

                    // Tell the camera to stop following the character
                    camera_events.send(CameraEvent::StopFollowing {
                        target_id: character_id,
                    });

                    // Tell the movement tools to manipulate the character
                    movement_target_events.send(MovementTargetEvent::SetTarget {
                        tool_id: movement_tool_id,
                        target: MovementTarget::Character,
                    });
                }
            }
        }

        if tool_actions.just_pressed(FocusToolAction::FocusMainWindow) {
            // Announce acknowledgement
            info!("Focus main window");

            // Get window entity
            let Ok(window_handle) = window_query.get_single() else {
                error!("No primary window found");
                return;
            };

            // Get window handle
            let win32handle = match window_handle.window_handle {
                raw_window_handle::RawWindowHandle::Win32(handle) => handle,
                _ => panic!("Unsupported window handle"),
            };

            // Focus the window
            focus_window(win32handle.hwnd as isize);

            // Get the center of the window title bar
            if let Ok(position) = get_window_title_bar_center_position(win32handle.hwnd as isize) {
                // Update host cursor
                match set_cursor_position(position) {
                    Ok(_) => info!("Moved cursor to window title bar"),
                    Err(e) => error!("Failed to move cursor to window title bar: {:?}", e),
                }

                // Get toolbelt
                let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
                    warn!("Toolbelt should have a parent");
                    continue;
                };
                let (toolbelt_parent, _) = toolbelt;

                // Get character
                let Ok(character) = character_query.get_mut(toolbelt_parent.get()) else {
                    warn!("Toolbelt should have a character");
                    continue;
                };
                let (_, _, _, character_children) = character;

                // For each cursor
                for child in character_children.iter() {
                    if let Ok(mut cursor) = cursor_query.get_mut(*child) {
                        // Set the desired position
                        info!("Set cursor position to window title bar center");
                        cursor.desired_position = Some(position.clone().as_vec2().neg_y());
                    }
                }
            }
        } else if tool_actions.just_released(FocusToolAction::FocusMainWindow) {
            // Get toolbelt
            let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
                warn!("Toolbelt should have a parent");
                continue;
            };
            let (toolbelt_parent, _) = toolbelt;

            // Get character
            let Ok(character) = character_query.get_mut(toolbelt_parent.get()) else {
                warn!("Toolbelt should have a character");
                continue;
            };
            let (_, _, _, character_children) = character;

            // For each cursor
            for child in character_children.iter() {
                if let Ok(mut cursor) = cursor_query.get_mut(*child) {
                    // Clear the desired position
                    info!("Cleared cursor position");
                    cursor.desired_position = None;
                }
            }
        }
    }
}
