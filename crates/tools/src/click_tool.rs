use std::thread;

use bevy::audio::Volume;
use bevy::audio::VolumeLevel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use cursor_hero_pointer_types::prelude::*;

use leafwing_input_manager::prelude::*;

use crossbeam_channel::bounded;
use crossbeam_channel::Sender;
use cursor_hero_character::character_plugin::Character;

use cursor_hero_winutils::win_mouse::left_mouse_down;
use cursor_hero_winutils::win_mouse::left_mouse_up;
use cursor_hero_winutils::win_mouse::right_mouse_down;
use cursor_hero_winutils::win_mouse::right_mouse_up;

use cursor_hero_toolbelt_types::prelude::*;

use crate::prelude::*;

pub struct ClickToolPlugin;

impl Plugin for ClickToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ClickTool>()
            .add_plugins(InputManagerPlugin::<ClickToolAction>::default())
            .add_systems(Startup, spawn_worker_thread)
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, Reflect, Default)]
struct ClickTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Default { toolbelt_id }
        | PopulateToolbeltEvent::Keyboard { toolbelt_id } = event
        {
            ToolSpawnConfig::<ClickTool, ClickToolAction>::new(ClickTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Send mouse clicks")
                .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum ClickToolAction {
    LeftClick,
    RightClick,
}
impl ClickToolAction {
    fn get_audio_path(&self, motion: Motion) -> &'static str {
        match (self, motion) {
            (Self::LeftClick, Motion::Down) => "sounds/mouse1down.ogg",
            (Self::LeftClick, Motion::Up) => "sounds/mouse1up.ogg",
            (Self::RightClick, Motion::Down) => "sounds/mouse2down.ogg",
            (Self::RightClick, Motion::Up) => "sounds/mouse2up.ogg",
        }
    }
    fn get_thread_message(&self, motion: Motion) -> ClickThreadMessage {
        match (self, motion) {
            (Self::LeftClick, Motion::Down) => ClickThreadMessage::LeftMouse(Motion::Down),
            (Self::LeftClick, Motion::Up) => ClickThreadMessage::LeftMouse(Motion::Up),
            (Self::RightClick, Motion::Down) => ClickThreadMessage::RightMouse(Motion::Down),
            (Self::RightClick, Motion::Up) => ClickThreadMessage::RightMouse(Motion::Up),
        }
    }
}
impl From<ClickToolAction> for Way {
    fn from(action: ClickToolAction) -> Self {
        match action {
            ClickToolAction::LeftClick => Way::Left,
            ClickToolAction::RightClick => Way::Right,
        }
    }
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
    fn default_wheel_gamepad_binding(&self) -> UserInput {
        match self {
            Self::LeftClick => GamepadButtonType::RightTrigger.into(),
            Self::RightClick => GamepadButtonType::LeftTrigger.into(),
        }
    }

    fn default_wheel_keyboard_binding(&self) -> UserInput {
        match self {
            Self::LeftClick => MouseButton::Left.into(),
            Self::RightClick => MouseButton::Right.into(),
        }
    }
    fn keyboard_wheel_gamepad_binding(&self) -> UserInput {
        match self {
            Self::LeftClick => GamepadButtonType::RightThumb.into(),
            Self::RightClick => GamepadButtonType::LeftThumb.into(),
        }
    }

    fn keyboard_wheel_keyboard_binding(&self) -> UserInput {
        match self {
            Self::LeftClick => MouseButton::Left.into(),
            Self::RightClick => MouseButton::Right.into(),
        }
    }
}

impl ToolAction for ClickToolAction {
    fn default_input_map(event: &PopulateToolbeltEvent) -> Option<InputMap<ClickToolAction>> {
        match event {
            PopulateToolbeltEvent::Default { .. } => Some(Self::with_defaults(
                Self::default_wheel_gamepad_binding,
                Self::default_wheel_keyboard_binding,
            )),
            PopulateToolbeltEvent::Keyboard { .. } => Some(Self::with_defaults(
                Self::keyboard_wheel_gamepad_binding,
                Self::keyboard_wheel_keyboard_binding,
            )),
            _ => None,
        }
    }
}

fn spawn_worker_thread(mut commands: Commands) {
    let (tx, rx) = bounded::<_>(10);
    commands.insert_resource(ClickBridge { sender: tx });
    thread::spawn(move || loop {
        let (action, x, y) = match rx.recv() {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to receive thread message, exiting: {:?}", e);
                break;
            }
        };
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

#[allow(clippy::too_many_arguments)]
fn handle_input(
    mut commands: Commands,
    tools: Query<(&ActionState<ClickToolAction>, &Parent), (With<ActiveTool>, With<ClickTool>)>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<&Children, With<Character>>,
    pointers: Query<(Entity, &GlobalTransform), With<Pointer>>,
    bridge: ResMut<ClickBridge>,
    asset_server: Res<AssetServer>,
    mut tool_click_event_writer: EventWriter<ToolClickEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for tool in tools.iter() {
        let (tool_actions, tool_parent) = tool;

        if !ClickToolAction::variants()
            .any(|action| tool_actions.just_pressed(action) || tool_actions.just_released(action))
        {
            continue;
        }

        let Ok(toolbelt) = toolbelts.get(tool_parent.get()) else {
            warn!("Tool not inside a toolbelt?");
            continue;
        };
        let toolbelt_parent = toolbelt;

        let Ok(character) = characters.get(toolbelt_parent.get()) else {
            warn!("Toolbelt parent not a character?");
            continue;
        };
        let character_children = character;

        let Some(pointer) = character_children
            .iter()
            .filter_map(|x| pointers.get(*x).ok())
            .next()
        else {
            //TODO: warn if more than one pointer found
            warn!("Character {:?} missing a pointer?", toolbelt_parent.get());
            debug!("Character children: {:?}", character_children);
            continue;
        };
        let (pointer_id, pointer_transform) = pointer;
        let pointer_pos = pointer_transform.translation();

        let window = window_query.get_single().expect("Need a single window");
        if window.cursor_position().is_some() {
            // The host cursor is over the window
            // Perform virtual click instead of sending a message to the worker thread
            // debug!("Performing virtual click");
            for action in ClickToolAction::variants() {
                if tool_actions.just_pressed(action) {
                    info!("{:?} pressed", action);
                    commands.spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(pointer_pos),
                            ..default()
                        },
                        Name::new("Click sound"),
                        AudioBundle {
                            source: asset_server.load(action.get_audio_path(Motion::Down)),
                            settings: PlaybackSettings::DESPAWN
                                .with_spatial(true)
                                .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                        },
                    ));
                    tool_click_event_writer.send(ToolClickEvent::Pressed {
                        pointer_id,
                        way: action.into(),
                    });
                }
                if tool_actions.just_released(action) {
                    info!("{:?} released", action);
                    commands.spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(pointer_pos),
                            ..default()
                        },
                        Name::new("Click sound"),
                        AudioBundle {
                            source: asset_server.load(action.get_audio_path(Motion::Up)),
                            settings: PlaybackSettings::DESPAWN
                                .with_spatial(true)
                                .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                        },
                    ));
                    tool_click_event_writer.send(ToolClickEvent::Released {
                        pointer_id,
                        way: action.into(),
                    });
                }
            }
        } else {
            // The host cursor is outside the window
            // Send a message to the worker thread
            // debug!("Performing host click");
            for action in ClickToolAction::variants() {
                if tool_actions.just_pressed(action) {
                    info!("{:?} pressed", action);
                    match bridge.sender.send((
                        action.get_thread_message(Motion::Down),
                        pointer_pos.x as i32,
                        -pointer_pos.y as i32,
                    )) {
                        Ok(_) => {
                            commands.spawn((
                                SpatialBundle {
                                    transform: Transform::from_translation(pointer_pos),
                                    ..default()
                                },
                                Name::new("Click sound"),
                                AudioBundle {
                                    source: asset_server.load(action.get_audio_path(Motion::Down)),
                                    settings: PlaybackSettings::DESPAWN
                                        .with_spatial(true)
                                        .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                                },
                            ));
                        }
                        Err(e) => {
                            error!("Failed to send click: {:?}", e);
                        }
                    }
                }
                if tool_actions.just_released(action) {
                    info!("{:?} released", action);
                    match bridge.sender.send((
                        action.get_thread_message(Motion::Up),
                        pointer_pos.x as i32,
                        -pointer_pos.y as i32,
                    )) {
                        Ok(_) => {
                            commands.spawn((
                                SpatialBundle {
                                    transform: Transform::from_translation(pointer_pos),
                                    ..default()
                                },
                                Name::new("Click sound"),
                                AudioBundle {
                                    source: asset_server.load(action.get_audio_path(Motion::Up)),
                                    settings: PlaybackSettings::DESPAWN
                                        .with_spatial(true)
                                        .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                                },
                            ));
                        }

                        Err(e) => {
                            error!("Failed to send click: {:?}", e);
                        }
                    }
                }
            }
        }
    }
}
