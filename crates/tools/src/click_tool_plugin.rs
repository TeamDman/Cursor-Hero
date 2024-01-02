use std::thread;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crossbeam_channel::bounded;
use crossbeam_channel::Sender;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_pointer::pointer_plugin::Pointer;
use cursor_hero_winutils::win_mouse::left_mouse_down;
use cursor_hero_winutils::win_mouse::left_mouse_up;
use cursor_hero_winutils::win_mouse::right_mouse_down;
use cursor_hero_winutils::win_mouse::right_mouse_up;

use cursor_hero_toolbelt::types::*;

pub struct ClickToolPlugin;

impl Plugin for ClickToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ClickTool>()
            .add_plugins(InputManagerPlugin::<ClickToolAction>::default())
            .add_systems(Startup, spawn_worker_thread)
            .add_systems(
                Update,
                (spawn_tool_event_responder_update_system, handle_input),
            );
    }
}

#[derive(Component, Reflect)]
pub struct ClickTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ClickToolAction {
    LeftClick,
    RightClick,
}

#[derive(Debug)]
enum Motion {
    Up,
    Down,
}

#[derive(Debug)]
enum ClickThreadMessage {
    LeftMouse(Motion),
    RightMouse(Motion),
}

#[derive(Resource)]
struct ClickBridge {
    pub sender: Sender<(ClickThreadMessage, i32, i32)>,
}

impl ClickToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::LeftClick => GamepadButtonType::RightTrigger.into(),
            Self::RightClick => GamepadButtonType::LeftTrigger.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::LeftClick => KeyCode::ControlLeft.into(),
            Self::RightClick => KeyCode::ControlRight.into(),
        }
    }

    fn default_input_map() -> InputMap<ClickToolAction> {
        let mut input_map = InputMap::default();

        for variant in ClickToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

fn spawn_worker_thread(mut commands: Commands) {
    let (tx, rx) = bounded::<_>(10);
    commands.insert_resource(ClickBridge { sender: tx });
    thread::spawn(move || loop {
        let (action, x, y) = rx.recv().unwrap();
        debug!("Worker received click: {:?} {} {}", action, x, y);
        match match action {
            ClickThreadMessage::LeftMouse(Motion::Down) => left_mouse_down(),
            ClickThreadMessage::LeftMouse(Motion::Up) => left_mouse_up(),
            ClickThreadMessage::RightMouse(Motion::Down) => right_mouse_down(),
            ClickThreadMessage::RightMouse(Motion::Up) => right_mouse_up(),
        } {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to handle event {:?}: {:?}", action, e);
            }
        }
    });
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
                            name: Name::new(format!("Click Tool")),
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server.load("textures/tool_mouse.png"),
                                ..default()
                            },
                            ..default()
                        },
                        InputManagerBundle::<ClickToolAction> {
                            input_map: ClickToolAction::default_input_map(),
                            ..default()
                        },
                        ToolActiveTag,
                        ClickTool,
                    ));
                });
                info!("Added tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

fn handle_input(
    tools: Query<(
        &ActionState<ClickToolAction>,
        Option<&ToolActiveTag>,
        &Parent,
    )>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<&Children, With<Character>>,
    pointers: Query<&GlobalTransform, With<Pointer>>,
    // window: Query<(Entity, &Window), With<PrimaryWindow>>,
    // winit_windows: NonSendMut<WinitWindows>,
    bridge: ResMut<ClickBridge>,
) {
    for (t_act, t_enabled, t_parent) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }
        let c_kids = characters
            .get(
                toolbelts
                    .get(t_parent.get())
                    .expect("Toolbelt should have a parent")
                    .get(),
            )
            .expect("Toolbelt should have a character");
        let p = c_kids
            .iter()
            .filter_map(|x| pointers.get(*x).ok())
            .next()
            .expect("Character should have a pointer");
        let p_pos = p.translation();
        if t_act.just_pressed(ClickToolAction::LeftClick) {
            info!("Left click pressed");
            match bridge.sender.send((
                ClickThreadMessage::LeftMouse(Motion::Down),
                p_pos.x as i32,
                -p_pos.y as i32,
            )) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to send click: {:?}", e);
                }
            }
        }
        if t_act.just_released(ClickToolAction::LeftClick) {
            info!("Left click released");
            match bridge.sender.send((
                ClickThreadMessage::LeftMouse(Motion::Up),
                p_pos.x as i32,
                -p_pos.y as i32,
            )) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to send click: {:?}", e);
                }
            }
        }
        if t_act.just_pressed(ClickToolAction::RightClick) {
            info!("Right click pressed");
            match bridge.sender.send((
                ClickThreadMessage::RightMouse(Motion::Down),
                p_pos.x as i32,
                -p_pos.y as i32,
            )) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to send click: {:?}", e);
                }
            }
        }
        if t_act.just_released(ClickToolAction::RightClick) {
            info!("Right click released");
            match bridge.sender.send((
                ClickThreadMessage::RightMouse(Motion::Up),
                p_pos.x as i32,
                -p_pos.y as i32,
            )) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to send click: {:?}", e);
                }
            }
        }

        // winit_windows
        //     .get_window(window.single().0)
        //     .map(|w| w.focus_window());
    }
}
