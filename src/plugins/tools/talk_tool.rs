use std::thread;

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use leafwing_input_manager::prelude::*;

use crate::{
    plugins::{character_plugin::Character, pointer_plugin::Pointer},
    utils::win_mouse::{
        left_click, left_mouse_down, left_mouse_up, right_click, right_mouse_up, ui_left_click,
        ui_right_click, right_mouse_down, press_f23_key, release_f23_key,
    },
};
use crossbeam_channel::{bounded, Receiver, Sender};

use super::super::toolbelt::types::*;

pub struct TalkToolPlugin;

impl Plugin for TalkToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TalkTool>()
            .add_plugins(InputManagerPlugin::<ToolAction>::default())
            .add_systems(Startup, spawn_worker_thread)
            .add_systems(
                Update,
                (spawn_tool_event_responder_update_system, handle_input),
            );
    }
}

#[derive(Component, Reflect)]
pub struct TalkTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ToolAction {
    Listen,
}

#[derive(Debug)]
enum Motion {
    Up,
    Down,
}

#[derive(Debug)]
enum ThreadMessage {
    ListenButton(Motion),
}

#[derive(Resource)]
struct Bridge {
    pub sender: Sender<ThreadMessage>,
}

impl ToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Listen => GamepadButtonType::West.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Listen => KeyCode::ShiftRight.into(),
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

fn spawn_worker_thread(mut commands: Commands) {
    let (tx, rx) = bounded::<_>(10);
    commands.insert_resource(Bridge { sender: tx });
    thread::spawn(move || loop {
        let action = rx.recv().unwrap();
        debug!("Worker received thread message: {:?}", action);
        match (match action {
            ThreadMessage::ListenButton(Motion::Down) => press_f23_key(),
            ThreadMessage::ListenButton(Motion::Up) => release_f23_key(),
        }) {
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
                            name: Name::new(format!("Talk Tool")),
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server.load("textures/tool_talk.png"),
                                ..default()
                            },
                            ..default()
                        },
                        InputManagerBundle::<ToolAction> {
                            input_map: ToolAction::default_input_map(),
                            ..default()
                        },
                        ToolActiveTag,
                        TalkTool,
                    ));
                });
                info!("Added talk tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

fn handle_input(
    tools: Query<(
        &ActionState<ToolAction>,
        Option<&ToolActiveTag>,
        &Parent,
    )>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<&Children, With<Character>>,
    pointers: Query<&GlobalTransform, With<Pointer>>,
    window: Query<(Entity, &Window), With<PrimaryWindow>>,
    winit_windows: NonSendMut<WinitWindows>,
    mut bridge: ResMut<Bridge>,
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
        if t_act.just_pressed(ToolAction::Listen) {
            info!("Listen button pressed");
            match bridge
                .sender
                .send(ThreadMessage::ListenButton(Motion::Down))
            {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to send thread message: {:?}", e);
                }
            }
        }
        if t_act.just_released(ToolAction::Listen) {
            info!("Listen button released");
            match bridge
                .sender
                .send(ThreadMessage::ListenButton(Motion::Up))
            {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to send thread message: {:?}", e);
                }
            }
        }
    }
}
