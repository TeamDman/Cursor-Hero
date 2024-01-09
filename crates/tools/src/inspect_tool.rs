use std::thread;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use crossbeam_channel::Receiver;
use cursor_hero_hover::hover_ui_automation_plugin::get_element_info;
use cursor_hero_physics::damping_plugin::MovementDamping;
use leafwing_input_manager::prelude::*;

use crossbeam_channel::bounded;
use crossbeam_channel::Sender;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_hover::hover_ui_automation_plugin::ElementInfo;
use cursor_hero_pointer::pointer_plugin::Pointer;
use cursor_hero_winutils::win_mouse::find_element_at;

use cursor_hero_toolbelt::types::*;

pub struct InspectToolPlugin;

impl Plugin for InspectToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InspectTool>()
            .add_plugins(InputManagerPlugin::<InspectToolAction>::default())
            .add_systems(Startup, spawn_worker_thread)
            .add_systems(
                Update,
                (
                    spawn_tool_event_responder_update_system,
                    handle_input,
                    handle_replies,
                ),
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
enum ThreadboundMessage {
    PrintUnderMouse(i32, i32),
}
#[derive(Debug)]
enum GameboundMessage {
    ElementDetails(ElementInfo),
}

#[derive(Resource)]
struct Bridge {
    pub sender: Sender<ThreadboundMessage>,
    pub receiver: Receiver<GameboundMessage>,
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

fn process_thread_message(
    action: ThreadboundMessage,
    reply_tx: &Sender<GameboundMessage>,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ThreadboundMessage::PrintUnderMouse(x, y) => {
            debug!("Worker received click: {:?} {} {}", action, x, y);

            let elem = find_element_at(x, y)?;
            info!("{} - {}", elem.get_classname()?, elem.get_name()?);

            let id = elem.get_automation_id()?;
            info!("Automation ID: {}", id);
            let info = get_element_info(elem)?;
            reply_tx.send(GameboundMessage::ElementDetails(info))?;
        }
    }

    Ok(())
}

fn spawn_worker_thread(mut commands: Commands) {
    let (tx, rx) = bounded::<_>(10);
    let (reply_tx, reply_rx) = bounded::<_>(10); // New channel for replies

    commands.insert_resource(Bridge {
        sender: tx,
        receiver: reply_rx,
    });
    thread::spawn(move || loop {
        let action = rx.recv().unwrap();
        if let Err(e) = process_thread_message(action, &reply_tx) {
            error!("Failed to process thread message: {:?}", e);
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
            match bridge.sender.send(ThreadboundMessage::PrintUnderMouse(
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

fn handle_replies(mut commands: Commands, bridge: Res<Bridge>) {
    while let Ok(msg) = bridge.receiver.try_recv() {
        match msg {
            GameboundMessage::ElementDetails(info) => {
                info!("Received element name: {}", info.name);
                let size = info.bounding_rect.max - info.bounding_rect.min;
                let mut position = (info.bounding_rect.min + size / 2.0).extend(20.0);
                position.y *= -1.0;
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(position),
                        sprite: Sprite {
                            custom_size: Some(size.clone()),
                            ..default()
                        },
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Collider::cuboid(size.x, size.y),
                    MovementDamping::default(),
                    Name::new(format!("Element - {}", info.name)),
                ));
            }
        }
    }
}
