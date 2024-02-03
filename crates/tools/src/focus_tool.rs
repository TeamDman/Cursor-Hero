use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use cursor_hero_camera::camera_plugin::FollowWithMainCamera;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_winutils::win_mouse::set_cursor_position;
use cursor_hero_winutils::win_window::get_window_title_bar_center_position;
use leafwing_input_manager::prelude::*;

use cursor_hero_camera::camera_plugin::CameraEvent;
use cursor_hero_character_types::prelude::*;
use cursor_hero_winutils::win_window::focus_window;

use cursor_hero_toolbelt_types::prelude::*;

use crate::movement_tool::MovementTarget;
use crate::movement_tool::MovementTargetEvent;
use crate::movement_tool::MovementTool;
use crate::prelude::*;

pub struct FocusToolPlugin;

impl Plugin for FocusToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FocusTool>()
            .add_plugins(InputManagerPlugin::<FocusToolAction>::default())
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}
#[derive(Component, Reflect, Default)]
struct FocusTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Default { toolbelt_id }
        | PopulateToolbeltEvent::Inspector { toolbelt_id } = event
        {
            ToolSpawnConfig::<FocusTool, FocusToolAction>::new(FocusTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Camera follows the character")
                .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum FocusToolAction {
    ToggleFollowCharacter,
    FocusMainWindow,
}

impl FocusToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::ToggleFollowCharacter => GamepadButtonType::LeftThumb.into(),
            Self::FocusMainWindow => GamepadButtonType::RightThumb.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::ToggleFollowCharacter => KeyCode::Space.into(),
            Self::FocusMainWindow => KeyCode::Home.into(),
        }
    }
}
impl ToolAction for FocusToolAction {
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<FocusToolAction>> {
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
    focus_tool_query: Query<(&ActionState<FocusToolAction>, &Parent), With<ActiveTool>>,
    movement_tool_query: Query<Entity, With<MovementTool>>,
    toolbelt_query: Query<(&Parent, &Children), With<Toolbelt>>,
    mut character_query: Query<
        (Entity, &mut Transform, Option<&FollowWithMainCamera>),
        (With<Character>, Without<MainCamera>),
    >,
    camera_query: Query<(Entity, &Transform), (With<MainCamera>, Without<Character>)>,
    window_query: Query<&RawHandleWrapper, With<PrimaryWindow>>,
    mut camera_events: EventWriter<CameraEvent>,
    mut movement_target_events: EventWriter<MovementTargetEvent>,
) {
    for tool in focus_tool_query.iter() {
        let (tool_actions, tool_parent) = tool;

        if tool_actions.just_pressed(FocusToolAction::ToggleFollowCharacter) {
            info!("Toggle follow character");
            let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
                warn!("Toolbelt should have a parent");
                continue;
            };

            let (toolbelt_parent, toolbelt_children) = toolbelt;
            let movement_tool_ids = toolbelt_children
                .iter()
                .filter_map(|child| movement_tool_query.get(*child).ok());

            let Ok(character) = character_query.get_mut(toolbelt_parent.get()) else {
                warn!("Toolbelt should have a character");
                continue;
            };
            let (character_id, mut character_transform, character_is_followed) = character;

            let camera = camera_query.single();
            let (camera_id, camera_transform) = camera;
            if character_is_followed.is_none() {
                camera_events.send(CameraEvent::BeginFollowing {
                    target_id: character_id,
                });
                movement_tool_ids.for_each(|id| {
                    movement_target_events.send(MovementTargetEvent::SetTarget {
                        tool_id: id,
                        target: MovementTarget::Character,
                    });
                });
                info!("Sent follow events");
                info!("Updating character to be at camera position");
                character_transform.translation = camera_transform.translation;
            } else {
                camera_events.send(CameraEvent::StopFollowing {
                    target_id: character_id,
                });
                movement_tool_ids.for_each(|id| {
                    movement_target_events.send(MovementTargetEvent::SetTarget {
                        tool_id: id,
                        target: MovementTarget::Camera(camera_id),
                    });
                });
                info!("Sent unfollow events");
            }
        }
        if tool_actions.just_pressed(FocusToolAction::FocusMainWindow) {
            info!("Focus main window");
            let Ok(window_handle) = window_query.get_single() else {
                error!("No primary window found");
                return;
            };
            let win32handle = match window_handle.window_handle {
                raw_window_handle::RawWindowHandle::Win32(handle) => handle,
                _ => panic!("Unsupported window handle"),
            };
            focus_window(win32handle.hwnd as isize);
            if let Ok(position) = get_window_title_bar_center_position(win32handle.hwnd as isize) {
                match set_cursor_position(position) {
                    Ok(_) => info!("Moved cursor to window title bar"),
                    Err(e) => error!("Failed to move cursor to window title bar: {:?}", e),
                }
            }
        }
    }
}
