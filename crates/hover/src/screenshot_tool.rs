use bevy_egui::EguiContext;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::hover_ui_automation_plugin::ElementInfo;
use crate::hover_ui_automation_plugin::get_element_info;
use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use cursor_hero_bevy::NegativeYVec2;
use cursor_hero_character_types::prelude::*;
use cursor_hero_environment_types::prelude::TrackEnvironmentTag;
use cursor_hero_floaty_nametag_types::prelude::*;
use cursor_hero_physics::damping_plugin::MovementDamping;
use cursor_hero_pointer_types::prelude::*;
use cursor_hero_screen::get_image::get_image;
use cursor_hero_screen::get_image::ScreensToImageParam;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::cube_tool::CubeToolInteractable;
use cursor_hero_tools::prelude::*;
use cursor_hero_winutils::ui_automation::find_element_at;
use cursor_hero_winutils::ui_automation::gather_elements_at;
use leafwing_input_manager::prelude::*;
use rand::Rng;
use rand::thread_rng;
use std::thread;

pub struct ScreenshotToolPlugin;

impl Plugin for ScreenshotToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InspectTool>()
            .add_plugins(InputManagerPlugin::<ScreenshotToolAction>::default())
            .add_systems(Startup, spawn_worker_thread)
            .add_systems(Update, (toolbelt_events, handle_input, handle_replies));
    }
}

#[derive(Component, Reflect, Default)]
struct InspectTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Inspector { toolbelt_id } = event {
            ToolSpawnConfig::<InspectTool, ScreenshotToolAction>::new(
                InspectTool,
                *toolbelt_id,
                event,
            )
            .guess_name(file!())
            .guess_image(file!(), &asset_server, "webp")
            .with_description("Turn UI elements into information.")
            .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum ScreenshotToolAction {
    Dupe,
    Print,
    Fracture,
}

impl ScreenshotToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Dupe => GamepadButtonType::RightTrigger.into(),
            Self::Print => GamepadButtonType::North.into(),
            Self::Fracture => GamepadButtonType::Select.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Dupe => MouseButton::Left.into(),
            Self::Print => MouseButton::Right.into(),
            Self::Fracture => KeyCode::G.into(),
        }
    }
}
impl ToolAction for ScreenshotToolAction {
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<ScreenshotToolAction>> {
        let mut input_map = InputMap::default();

        for variant in ScreenshotToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}

#[derive(Debug)]
enum ThreadboundMessage {
    Dupe { world_position: Vec3 },
    Print { world_position: Vec3 },
    Fracture { world_position: Vec3 },
}
#[derive(Debug)]
enum GameboundMessage {
    Dupe {
        element_info: ElementInfo,
        world_position: Vec3,
    },
    Print(ElementInfo),
    Fracture {
        data: Vec<(ElementInfo, usize)>,
        world_position: Vec3,
    },
}

#[derive(Resource)]
struct Bridge {
    pub sender: Sender<ThreadboundMessage>,
    pub receiver: Receiver<GameboundMessage>,
}
fn process_thread_message(
    action: ThreadboundMessage,
    reply_tx: &Sender<GameboundMessage>,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ThreadboundMessage::Dupe { world_position } => {
            let mouse_position = world_position.xy().neg_y().as_ivec2();
            debug!("Worker received click: {:?} {:?}", action, mouse_position);

            let elem = find_element_at(mouse_position)?;
            info!("{} - {}", elem.get_classname()?, elem.get_name()?);

            let id = elem.get_automation_id()?;
            info!("Automation ID: {}", id);
            let element_info = get_element_info(elem)?;
            reply_tx.send(GameboundMessage::Dupe {
                element_info,
                world_position,
            })?;
        }
        ThreadboundMessage::Print { world_position } => {
            let mouse_position = world_position.xy().neg_y().as_ivec2();
            debug!("Worker received click: {:?} {:?}", action, mouse_position);

            let elem = find_element_at(mouse_position)?;
            info!("{} - {}", elem.get_classname()?, elem.get_name()?);

            // Can we click on elements with this?
            // elem.send_keys(keys, interval) exists!

            // Send the info
            let id = elem.get_automation_id()?;
            info!("Automation ID: {}", id);
            let info = get_element_info(elem)?;
            reply_tx.send(GameboundMessage::Print(info))?;
        }
        ThreadboundMessage::Fracture { world_position } => {
            let mouse_position = world_position.xy().neg_y().as_ivec2();
            debug!("Worker received click: {:?} {:?}", action, mouse_position);

            let found = gather_elements_at(mouse_position)?;
            let data = found
                .into_iter()
                .filter_map(|(elem, depth)| get_element_info(elem).ok().map(|info| (info, depth)))
                .collect();
            reply_tx.send(GameboundMessage::Fracture {
                data,
                world_position,
            })?;
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
        let action = match rx.recv() {
            Ok(action) => action,
            Err(e) => {
                error!("Failed to receive thread message, exiting: {:?}", e);
                break;
            }
        };
        if let Err(e) = process_thread_message(action, &reply_tx) {
            error!("Failed to process thread message: {:?}", e);
        }
    });
}

fn handle_input(
    tools: Query<(&ActionState<ScreenshotToolAction>, &Parent), With<ActiveTool>>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<&Children, With<Character>>,
    pointers: Query<&GlobalTransform, With<Pointer>>,
    bridge: ResMut<Bridge>,
    egui_context_query: Query<&EguiContext, With<PrimaryWindow>>,
) {
    for tool in tools.iter() {
        let (tool_actions, tool_parent) = tool;

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
        let pointer_transform = pointer;
        let pointer_translation = pointer_transform.translation();
        let hovering_over_egui = egui_context_query
            .get_single()
            .ok()
            .map(|egui_context| egui_context.clone().get_mut().is_pointer_over_area())
            .unwrap_or(false);
        if hovering_over_egui {
            continue;
        }
        if tool_actions.just_pressed(ScreenshotToolAction::Dupe) {
            info!("PrintUnderMouse button");
            let msg = ThreadboundMessage::Dupe {
                world_position: pointer_translation,
            };
            if let Err(e) = bridge.sender.send(msg) {
                error!("Failed to send click: {:?}", e);
            }
        }
        if tool_actions.just_pressed(ScreenshotToolAction::Print) {
            info!("PrintUnderMouse button");
            let msg = ThreadboundMessage::Print {
                world_position: pointer_translation,
            };
            if let Err(e) = bridge.sender.send(msg) {
                error!("Failed to send click: {:?}", e);
            }
        }
        if tool_actions.just_pressed(ScreenshotToolAction::Fracture) {
            info!("FractureUnderMouse button");
            let msg = ThreadboundMessage::Fracture {
                world_position: pointer_translation,
            };
            if let Err(e) = bridge.sender.send(msg) {
                error!("Failed to send click: {:?}", e);
            }
        }
    }
}

fn handle_replies(
    mut commands: Commands,
    bridge: Res<Bridge>,
    access: ScreensToImageParam,
    asset_server: Res<AssetServer>,
) {
    while let Ok(msg) = bridge.receiver.try_recv() {
        match msg {
            GameboundMessage::Dupe { element_info, world_position } => {
                let Ok(image) = get_image(element_info.bounding_rect, &access) else {
                    continue;
                };
                let texture_handle = asset_server.add(image);

                // spawn the element image
                // let mut elem_center_pos = element_info.bounding_rect.center().extend(20.0);
                // elem_center_pos.y *= -1.0;

                // let size = element_info.bounding_rect.size();
                let size = Vec2::new(30.0,30.0);
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(world_position),
                        sprite: Sprite {
                            custom_size: Some(size),
                            ..default()
                        },
                        texture: texture_handle,
                        ..default()
                    },
                    AudioBundle {
                        source: asset_server.load("sounds/spring strung light 4.ogg"),
                        settings: PlaybackSettings::REMOVE.with_spatial(true),
                    },
                    FloatyName {
                        text: element_info.name.clone(),
                        vertical_offset: 40.0,
                        appearance: NametagAppearance::Databrick,
                    },
                    Hoverable,
                    Clickable,
                    CubeToolInteractable,
                    RigidBody::Dynamic,
                    TrackEnvironmentTag,
                    Collider::cuboid(size.x, size.y),
                    MovementDamping::default(),
                    Name::new(format!("Element - {}", element_info.name)),
                ));
            }
            GameboundMessage::Print(info) => {
                info!("Received info for element {:?}", info);
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("sounds/tape recorder eject 4.ogg"),
                        settings: PlaybackSettings::REMOVE,
                    },
                    Name::new(format!("SFX Element - {}", info.name)),
                ));
            }
            GameboundMessage::Fracture {
                data,
                world_position,
            } => {
                info!("Received info with {} elements", data.len());
                if !data.is_empty() {
                    commands.spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(world_position),
                            ..default()
                        },
                        AudioBundle {
                            source: asset_server.load("sounds/spring strung light 4.ogg"),
                            settings: PlaybackSettings::DESPAWN.with_spatial(true),
                        },
                        Name::new("Fracture Sound"),
                    ));
                }
                for (info, depth) in data {
                    // let Ok(image) = get_image(info.bounding_rect, &access) else {
                    //     continue;
                    // };
                    // let texture_handle = asset_server.add(image);

                    // spawn the element image
                    let mut elem_center_pos = info.bounding_rect.center().extend(depth as f32);
                    elem_center_pos.y *= -1.0;
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_translation(elem_center_pos),
                            sprite: Sprite {
                                custom_size: Some(info.bounding_rect.size()),
                                color: Color::hsl(thread_rng().gen_range(0.0..360.0), 0.5, 0.5),
                                ..default()
                            },
                            // texture: texture_handle,
                            ..default()
                        },
                        CubeToolInteractable,
                        RigidBody::Dynamic,
                        Collider::cuboid(info.bounding_rect.width(), info.bounding_rect.height()),
                        MovementDamping::default(),
                        Name::new(format!("Element - {}", info.name)),
                    ));
                }
            }
        }
    }
}
