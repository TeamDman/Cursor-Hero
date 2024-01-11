use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_movement::Movement;
use cursor_hero_winutils::win_mouse::set_cursor_position;
use cursor_hero_winutils::win_window::get_window_title_bar_center_position;
use leafwing_input_manager::prelude::*;

use cursor_hero_camera::camera_plugin::FollowWithCamera;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_character::character_plugin::CharacterColor;
use cursor_hero_winutils::win_window::focus_window;

use cursor_hero_toolbelt::types::*;

use crate::prelude::*;

pub struct FocusToolPlugin;

impl Plugin for FocusToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FocusTool>()
            .add_plugins(InputManagerPlugin::<FocusToolAction>::default())
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}
#[derive(Component, Reflect)]
struct FocusTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            ToolbeltEvent::PopulateDefaultToolbelt(toolbelt_id) => {
                spawn_action_tool::<FocusToolAction>(
                    file!(),
                    e,
                    &mut commands,
                    *toolbelt_id,
                    &asset_server,
                    FocusTool,
                );
            }
            _ => {}
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
            Self::ToggleFollowCharacter => KeyCode::Backslash.into(),
            Self::FocusMainWindow => KeyCode::Home.into(),
        }
    }
}
impl ToolAction for FocusToolAction {
    fn default_input_map() -> InputMap<FocusToolAction> {
        let mut input_map = InputMap::default();

        for variant in FocusToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

#[allow(clippy::type_complexity)]
fn handle_input(
    tools: Query<(
        &ActionState<FocusToolAction>,
        Option<&ToolActiveTag>,
        &Parent,
    )>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    mut characters: Query<
        (
            Entity,
            Option<&FollowWithCamera>,
            &mut Handle<ColorMaterial>,
        ),
        With<Character>,
    >,
    camera: Query<Entity, With<MainCamera>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&RawHandleWrapper, With<PrimaryWindow>>,
) {
    for (t_act, t_enabled, t_parent) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }
        if t_act.just_pressed(FocusToolAction::ToggleFollowCharacter) {
            info!("Toggle follow character");
            let toolbelt = toolbelts
                .get(t_parent.get())
                .expect("Toolbelt should have a parent");
            let character = characters
                .get_mut(toolbelt.get())
                .expect("Toolbelt should have a character");
            let (character_entity, character_is_followed, mut material) = character;

            if character_is_followed.is_none() {
                let mut character_commands = commands.entity(character_entity);
                // begin following character
                character_commands.insert(FollowWithCamera);
                // switch movement to character
                character_commands.insert(Movement::default());
                // remove movement from camera
                commands.entity(camera.single()).remove::<Movement>();
                // change color of character
                *material = materials.add(CharacterColor::FocusedWithCamera.as_material());
                info!("now following");
            } else {
                let mut character_commands = commands.entity(character_entity);
                // stop following character
                character_commands.remove::<FollowWithCamera>();
                // switch movement to camera
                character_commands.remove::<Movement>();
                // remove movement from character
                commands.entity(camera.single()).insert(Movement::default());
                // change color of character
                *material = materials.add(CharacterColor::Unfocused.as_material());
                info!("no longer following");
            }
        }
        if t_act.just_pressed(FocusToolAction::FocusMainWindow) {
            info!("Focus main window");
            let window_handle = window_query.get_single().expect("Need a single window");
            let win32handle = match window_handle.window_handle {
                raw_window_handle::RawWindowHandle::Win32(handle) => handle,
                _ => panic!("Unsupported window handle"),
            };
            focus_window(win32handle.hwnd as isize);
            if let Ok((x, y)) = get_window_title_bar_center_position(win32handle.hwnd as isize) {
                match set_cursor_position(x, y) {
                    Ok(_) => info!("Moved cursor to window title bar"),
                    Err(e) => error!("Failed to move cursor to window title bar: {:?}", e),
                }
            }
        }
    }
}
