use std::thread;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    plugins::{character_plugin::Character, pointer_plugin::Pointer},
    utils::win_mouse::{
        left_mouse_down, left_mouse_up, print_under_mouse, right_mouse_down, right_mouse_up,
    },
};
use crossbeam_channel::{bounded, Sender};

use super::super::toolbelt::types::*;

pub struct InspectToolPlugin;

impl Plugin for InspectToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InspectTool>()
            .add_plugins(InputManagerPlugin::<InspectToolAction>::default())
            .add_systems(Startup, spawn_worker_thread)
            .add_systems(
                Update,
                (spawn_tool_event_responder_update_system, handle_input),
            );
    }
}

#[derive(Component, Reflect)]
pub struct InspectTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum InspectToolAction {
    PrintUnderMouse,
}

#[derive(Debug)]
enum ThreadMessage {
    PrintUnderMouse,
}

#[derive(Resource)]
struct Bridge {
    pub sender: Sender<(ThreadMessage, i32, i32)>,
}

impl InspectToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::PrintUnderMouse => GamepadButtonType::RightTrigger.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::PrintUnderMouse => KeyCode::ControlLeft.into(),
        }
    }

    fn default_input_map() -> InputMap<InspectToolAction> {
        let mut input_map = InputMap::default();

        for variant in InspectToolAction::variants() {
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
        let (action, x, y) = rx.recv().unwrap();
        debug!("Worker received click: {:?} {} {}", action, x, y);
        match match action {
            ThreadMessage::PrintUnderMouse => print_under_mouse(x, y),
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
                            name: Name::new(format!("Inspect Tool")),
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server.load("textures/inspect_tool.png"),
                                ..default()
                            },
                            ..default()
                        },
                        InputManagerBundle::<InspectToolAction> {
                            input_map: InspectToolAction::default_input_map(),
                            ..default()
                        },
                        ToolActiveTag,
                        InspectTool,
                    ));
                });
                info!("Added tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

fn handle_input(
    tools: Query<(
        &ActionState<InspectToolAction>,
        Option<&ToolActiveTag>,
        &Parent,
    )>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<&Children, With<Character>>,
    pointers: Query<&GlobalTransform, With<Pointer>>,
    // window: Query<(Entity, &Window), With<PrimaryWindow>>,
    // winit_windows: NonSendMut<WinitWindows>,
    bridge: ResMut<Bridge>,
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
        if t_act.just_pressed(InspectToolAction::PrintUnderMouse) {
            info!("PrintUnderMouse button");
            match bridge.sender.send((
                ThreadMessage::PrintUnderMouse,
                p_pos.x as i32,
                -p_pos.y as i32,
            )) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to send click: {:?}", e);
                }
            }
        }
    }
}
