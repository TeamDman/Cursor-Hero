use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::RawHandleWrapper;
use cursor_hero_winutils::win_mouse::set_cursor_position;
use cursor_hero_winutils::win_window::get_window_title_bar_center_position;
use leafwing_input_manager::prelude::*;

use cursor_hero_camera::camera_plugin::FollowWithCamera;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_character::character_plugin::CharacterColor;
use cursor_hero_winutils::win_window::focus_window;

use cursor_hero_toolbelt::types::*;

pub struct FocusToolPlugin;

impl Plugin for FocusToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FocusTool>()
            .add_plugins(InputManagerPlugin::<ToolAction>::default())
            .add_systems(
                Update,
                (spawn_tool_event_responder_update_system, handle_input),
            );
    }
}

#[derive(Component, Reflect)]
pub struct FocusTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ToolAction {
    ToggleFollowCharacter,
    FocusMainWindow,
}

impl ToolAction {
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

    fn default_input_map() -> InputMap<ToolAction> {
        let mut input_map = InputMap::default();

        for variant in ToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

fn spawn_tool_event_responder_update_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            ToolbeltEvent::Populate(toolbelt_id) => {
                commands.entity(*toolbelt_id).with_children(|t_commands| {
                    t_commands.spawn((
                        ToolBundle {
                            name: Name::new(format!("Focus Tool")),
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server.load("textures/focus.png"),
                                ..default()
                            },
                            ..default()
                        },
                        InputManagerBundle::<ToolAction> {
                            input_map: ToolAction::default_input_map(),
                            ..default()
                        },
                        FocusTool,
                        ToolActiveTag,
                    ));
                });
                info!("Added tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn handle_input(
    tools: Query<(&ActionState<ToolAction>, Option<&ToolActiveTag>, &Parent)>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    mut characters: Query<
        (
            Entity,
            Option<&FollowWithCamera>,
            &mut Handle<ColorMaterial>,
        ),
        With<Character>,
    >,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&RawHandleWrapper, With<PrimaryWindow>>,
) {
    for (t_act, t_enabled, t_parent) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }
        if t_act.just_pressed(ToolAction::ToggleFollowCharacter) {
            info!("Toggle follow character");
            let toolbelt = toolbelts
                .get(t_parent.get())
                .expect("Toolbelt should have a parent");
            let character = characters
                .get_mut(toolbelt.get())
                .expect("Toolbelt should have a character");
            let (character_entity, character_is_followed, mut material) = character;

            if character_is_followed.is_none() {
                commands.entity(character_entity).insert(FollowWithCamera);
                *material = materials.add(CharacterColor::FocusedWithCamera.as_material());
                info!("now following");
            } else {
                commands
                    .entity(character_entity)
                    .remove::<FollowWithCamera>();
                *material = materials.add(CharacterColor::Unfocused.as_material());
                info!("no longer following");
            }
        }
        if t_act.just_pressed(ToolAction::FocusMainWindow) {
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
