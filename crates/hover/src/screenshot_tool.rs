use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui;
use bevy_egui::egui::Pos2;
use bevy_egui::EguiContext;
use bevy_egui::EguiContexts;
use bevy_inspector_egui::reflect_inspector::Context;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use crossbeam_channel::Sender;
use cursor_hero_bevy::prelude::NegativeYIVec3;
use cursor_hero_bevy::prelude::NegativeYVec2;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_character_types::prelude::*;
use cursor_hero_environment_types::prelude::TrackEnvironmentTag;
use cursor_hero_physics::damping_plugin::MovementDamping;
use cursor_hero_cursor_types::prelude::*;
use cursor_hero_screen::get_image::get_image;
use cursor_hero_screen::get_image::ScreensToImageParam;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::cube_tool::CubeToolInteractable;
use cursor_hero_tools::prelude::*;
use cursor_hero_ui_automation::prelude::find_element_at;
use cursor_hero_ui_automation::prelude::gather_elements_at;
use cursor_hero_ui_automation::prelude::gather_info_tree_ancestry_filtered;
use cursor_hero_ui_automation::prelude::DrillId;
use cursor_hero_ui_automation::prelude::ElementInfo;
use cursor_hero_worker::prelude::anyhow::Result;
use cursor_hero_worker::prelude::WorkerMessage;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerPlugin;
use leafwing_input_manager::prelude::*;
use rand::thread_rng;
use rand::Rng;

pub struct ScreenshotToolPlugin;

impl Plugin for ScreenshotToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorkerPlugin {
            config: WorkerConfig::<ThreadboundMessage, GameboundMessage, ()> {
                name: "screenshot_tool".to_string(),
                is_ui_automation_thread: true,
                handle_threadbound_message,
                ..default()
            },
        });
        app.register_type::<ScreenshotTool>();
        app.register_type::<ScreenshotBrick>();
        app.add_plugins(InputManagerPlugin::<ScreenshotToolAction>::default());
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, handle_input);
        app.add_systems(Update, handle_replies);
        // app.add_systems(Update, ui);
    }
}

#[derive(Debug, Reflect, Clone, Event)]
enum ThreadboundMessage {
    Capture { world_position: Vec3 },
    CaptureBrick { world_position: Vec3 },
    Print { world_position: Vec3 },
    Fracture { world_position: Vec3 },
}
impl WorkerMessage for ThreadboundMessage {}

#[derive(Debug, Reflect, Clone, Event)]
enum GameboundMessage {
    Capture {
        ui_tree: ElementInfo,
        hovered_element: ElementInfo,
        world_position: Vec3,
    },
    CaptureBrick {
        ui_tree: ElementInfo,
        hovered_element: ElementInfo,
        world_position: Vec3,
    },
    Print(ElementInfo),
    Fracture {
        data: Vec<(ElementInfo, usize)>,
        world_position: Vec3,
    },
}
impl WorkerMessage for GameboundMessage {}

#[derive(Component, Reflect, Default)]
struct ScreenshotTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum ScreenshotToolAction {
    Capture,
    CaptureBrick,
    Print,
    Fracture,
}
impl ScreenshotToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Capture => GamepadButtonType::RightTrigger.into(),
            Self::CaptureBrick => GamepadButtonType::South.into(),
            Self::Print => GamepadButtonType::North.into(),
            Self::Fracture => GamepadButtonType::Select.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Capture => MouseButton::Left.into(),
            Self::CaptureBrick => MouseButton::Middle.into(),
            Self::Print => MouseButton::Right.into(),
            Self::Fracture => KeyCode::G.into(),
        }
    }
}
impl ToolAction for ScreenshotToolAction {
    fn default_input_map(_event: &ToolbeltPopulateEvent) -> Option<InputMap<ScreenshotToolAction>> {
        let mut input_map = InputMap::default();

        for variant in ScreenshotToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}

#[derive(Reflect, Default)]
struct ScreenshotBrickEguiState {
    selected: Option<DrillId>,
    expanded: Vec<DrillId>,
}

#[derive(Component, Reflect)]
struct ScreenshotBrick {
    ui_tree: ElementInfo,
    hero_element: ElementInfo,
    egui_state: ScreenshotBrickEguiState,
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        if event.loadout == ToolbeltLoadout::Default {
            ToolSpawnConfig::<ScreenshotTool, ScreenshotToolAction>::new(
                ScreenshotTool,
                event.id,
                event,
            )
            .with_src_path(file!().into())
            .guess_name(file!())
            .guess_image(file!(), &asset_server, "webp")
            .with_description("Turn UI elements into information.")
            .with_starting_state(StartingState::Inactive)
            .spawn(&mut commands);
        }
    }
}

fn handle_input(
    tools: Query<(&ActionState<ScreenshotToolAction>, &Parent), With<ActiveTool>>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<&Children, With<Character>>,
    cursors: Query<&GlobalTransform, With<Cursor>>,
    mut bridge: EventWriter<ThreadboundMessage>,
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

        let Some(cursor) = character_children
            .iter()
            .filter_map(|x| cursors.get(*x).ok())
            .next()
        else {
            //TODO: warn if more than one cursor found
            warn!("Character {:?} missing a cursor?", toolbelt_parent.get());
            debug!("Character children: {:?}", character_children);
            continue;
        };
        let cursor_transform = cursor;
        let cursor_translation = cursor_transform.translation();
        let hovering_over_egui = egui_context_query
            .get_single()
            .ok()
            .map(|egui_context| egui_context.clone().get_mut().is_pointer_over_area())
            .unwrap_or(false);
        if hovering_over_egui {
            continue;
        }
        if tool_actions.just_pressed(ScreenshotToolAction::Capture) {
            info!("Capture");
            let msg = ThreadboundMessage::Capture {
                world_position: cursor_translation,
            };
            bridge.send(msg);
        }
        if tool_actions.just_pressed(ScreenshotToolAction::CaptureBrick) {
            info!("CaptureBrick");
            let msg = ThreadboundMessage::CaptureBrick {
                world_position: cursor_translation,
            };
            bridge.send(msg);
        }
        if tool_actions.just_pressed(ScreenshotToolAction::Print) {
            info!("Print");
            let msg = ThreadboundMessage::Print {
                world_position: cursor_translation,
            };
            bridge.send(msg);
        }
        if tool_actions.just_pressed(ScreenshotToolAction::Fracture) {
            info!("Fracture");
            let msg = ThreadboundMessage::Fracture {
                world_position: cursor_translation,
            };
            bridge.send(msg);
        }
    }
}

fn handle_threadbound_message(
    msg: &ThreadboundMessage,
    reply_tx: &Sender<GameboundMessage>,
    _state: &mut (),
) -> Result<()> {
    match msg {
        ThreadboundMessage::Capture { world_position }
        | ThreadboundMessage::CaptureBrick { world_position } => {
            let mouse_position = world_position.xy().neg_y().as_ivec2();
            debug!("Worker received click: {:?} {:?}", msg, mouse_position);

            let elem = find_element_at(mouse_position)?;
            info!("{} - {}", elem.get_classname()?, elem.get_name()?);

            let id = elem.get_automation_id()?;
            info!("Automation ID: {}", id);
            let gathered = gather_info_tree_ancestry_filtered(elem)?;
            // debug!("Element info: {:?}", element_info);
            let msg = match msg {
                ThreadboundMessage::Capture { world_position } => GameboundMessage::Capture {
                    ui_tree: gathered.ui_tree,
                    hovered_element: gathered.start_info,
                    world_position: *world_position,
                },
                ThreadboundMessage::CaptureBrick { world_position } => {
                    GameboundMessage::CaptureBrick {
                        ui_tree: gathered.ui_tree,
                        hovered_element: gathered.start_info,
                        world_position: *world_position,
                    }
                }
                _ => unreachable!(),
            };
            reply_tx.send(msg)?;
        }
        ThreadboundMessage::Print { world_position } => {
            let mouse_position = world_position.xy().neg_y().as_ivec2();
            debug!("Worker received click: {:?} {:?}", msg, mouse_position);

            let elem = find_element_at(mouse_position)?;
            info!("{:?}", elem);

            // Can we click on elements with this?
            // elem.send_keys(keys, interval) exists!

            // Send the info
            let id = elem.get_automation_id()?;
            info!("Automation ID: {}", id);
            let gathered = gather_info_tree_ancestry_filtered(elem)?;
            reply_tx.send(GameboundMessage::Print(gathered.ui_tree))?;
        }
        ThreadboundMessage::Fracture { world_position } => {
            let mouse_position = world_position.xy().neg_y().as_ivec2();
            debug!("Worker received click: {:?} {:?}", msg, mouse_position);

            let found = gather_elements_at(mouse_position)?;
            let data = found
                .into_iter()
                .filter_map(|(elem, depth)| {
                    gather_info_tree_ancestry_filtered(elem)
                        .ok()
                        .map(|info| (info.start_info, depth))
                })
                .collect();
            reply_tx.send(GameboundMessage::Fracture {
                data,
                world_position: *world_position,
            })?;
        }
    }

    Ok(())
}

fn handle_replies(
    mut commands: Commands,
    mut bridge: EventReader<GameboundMessage>,
    screen_access: ScreensToImageParam,
    asset_server: Res<AssetServer>,
) {
    for msg in bridge.read() {
        match &msg {
            GameboundMessage::Capture {
                ui_tree,
                hovered_element,
                world_position,
            }
            | GameboundMessage::CaptureBrick {
                ui_tree,
                hovered_element,
                world_position,
            } => {
                let (size, pos, texture_region) = match msg {
                    GameboundMessage::Capture { .. } => (
                        hovered_element.bounding_rect.size().as_vec2(),
                        hovered_element
                            .bounding_rect
                            .center()
                            .extend(20)
                            .neg_y()
                            .as_vec3(),
                        hovered_element.bounding_rect,
                    ),
                    GameboundMessage::CaptureBrick { .. } => (
                        hovered_element.bounding_rect.size().as_vec2().normalize() * 60.0,
                        *world_position,
                        hovered_element.bounding_rect,
                    ),
                    _ => unreachable!(),
                };
                spawn_brick(
                    &mut commands,
                    ui_tree,
                    hovered_element,
                    size,
                    pos,
                    texture_region,
                    &screen_access,
                    &asset_server,
                );
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
                            transform: Transform::from_translation(*world_position),
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
                    let mut elem_center_pos =
                        info.bounding_rect.center().as_vec2().extend(*depth as f32);
                    elem_center_pos.y *= -1.0;
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_translation(elem_center_pos),
                            sprite: Sprite {
                                custom_size: Some(info.bounding_rect.size().as_vec2()),
                                color: Color::hsl(thread_rng().gen_range(0.0..360.0), 0.5, 0.5),
                                ..default()
                            },
                            // texture: texture_handle,
                            ..default()
                        },
                        CubeToolInteractable,
                        RigidBody::Dynamic,
                        Collider::cuboid(
                            info.bounding_rect.width() as f32,
                            info.bounding_rect.height() as f32,
                        ),
                        MovementDamping::default(),
                        Name::new(format!("Element - {}", info.name)),
                    ));
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_brick(
    commands: &mut Commands,
    ui_tree: &ElementInfo,
    hero_element: &ElementInfo,
    size: Vec2,
    pos: Vec3,
    texture_region: IRect,
    screen_access: &ScreensToImageParam,
    asset_server: &Res<AssetServer>,
) {
    let Ok(image) = get_image(texture_region, screen_access) else {
        return;
    };
    let texture_handle = asset_server.add(image);
    let expanded = ui_tree
        .get_descendents()
        .iter()
        .chain([ui_tree].iter())
        .filter(|x| x.children.is_some())
        .map(|x| x.drill_id.clone())
        .collect();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(pos),
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
        // FloatyName {
        //     text: element_info.name.clone(),
        //     vertical_offset: 40.0,
        //     appearance: NametagAppearance::Databrick,
        // },
        Hoverable,
        Clickable,
        CubeToolInteractable,
        RigidBody::Dynamic,
        TrackEnvironmentTag,
        ScreenshotBrick {
            ui_tree: ui_tree.clone(),
            hero_element: hero_element.clone(),
            egui_state: ScreenshotBrickEguiState {
                selected: Some(hero_element.drill_id.clone()),
                expanded,
            },
        },
        Collider::cuboid(size.x, size.y),
        MovementDamping::default(),
        Name::new(format!("Element - {}", hero_element.name)),
    ));
}

fn ui(
    mut commands: Commands,
    screen_access: ScreensToImageParam,
    asset_server: Res<AssetServer>,
    mut contexts: EguiContexts,
    mut brick_query: Query<(
        Entity,
        &mut ScreenshotBrick,
        &Sprite,
        &Name,
        &GlobalTransform,
    )>,
    camera_query: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
    type_registry: Res<AppTypeRegistry>,
) {
    let Ok(camera) = camera_query.get_single() else {
        warn!("No camera found");
        return;
    };
    let (camera_transform, camera) = camera;

    let ctx = contexts.ctx_mut();
    // let scale = (camera_transform.compute_transform().scale.x * 1.0).round();
    // debug!("Scale: {}", scale);
    // ctx.set_zoom_factor(scale);

    if brick_query.is_empty() {
        return;
    }

    let mut cx = Context {
        world: None,
        queue: None,
    };

    let type_registry = type_registry.0.clone();
    let type_registry = type_registry.read();

    let mut inspector = InspectorUi::for_bevy(&type_registry, &mut cx);

    for brick in brick_query.iter_mut() {
        let (brick_id, mut brick, brick_sprite, brick_name, brick_global_transform) = brick;
        let brick_global_translation = brick_global_transform.translation();
        let popout_pos = brick_global_translation
            + Vec3::new(
                brick_sprite.custom_size.unwrap_or_default().x + 50.0,
                0.0,
                0.0,
            );
        let egui_pos = camera
            .world_to_viewport(camera_transform, brick_global_translation)
            .unwrap_or_default();

        let id = egui::Id::new(brick_id);

        egui::Window::new(brick_name.chars().take(64).collect::<String>())
            .id(id)
            .fixed_pos(Pos2::new(egui_pos.x, egui_pos.y))
            .default_width(1200.0)
            // .resizable(true)
            .show(ctx, |ui| {
                egui::SidePanel::left(id.with("tree"))
                    .resizable(true)
                    .width_range(100.0..=4000.0)
                    .default_width(600.0)
                    .show_inside(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("UI Tree");
                        });
                        egui::ScrollArea::both().show(ui, |ui| {
                            let id = id.with(brick.ui_tree.runtime_id.clone());

                            let mut temp_egui_state = std::mem::take(&mut brick.egui_state);
                            let mut temp_info = std::mem::take(&mut brick.ui_tree);
                            ui_for_element_info(
                                &mut temp_egui_state,
                                id,
                                &mut commands,
                                &screen_access,
                                &asset_server,
                                ui,
                                &mut temp_info,
                                &mut inspector,
                                &popout_pos,
                            );
                            brick.egui_state = temp_egui_state;
                            brick.ui_tree = temp_info;

                            ui.allocate_space(ui.available_size());
                        });
                    });

                egui::TopBottomPanel::bottom(id.with("invisible bottom panel"))
                    .show_separator_line(false)
                    .show_inside(ui, |_| ());

                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.heading("AHOY!");
                    let id = brick.egui_state.selected.clone();
                    if let Some(id) = id
                        && let Some(x) = brick.ui_tree.lookup_drill_id_mut(id)
                    {
                        inspector.ui_for_reflect(x, ui);
                    }
                    // inspector.ui_for_reflect_readonly(&data, ui);
                });
            });
    }
}

#[derive(Reflect, Debug)]
struct ElementUIData {
    runtime_id: String,
    frick: String,
}

#[allow(clippy::too_many_arguments)]
fn ui_for_element_info(
    state: &mut ScreenshotBrickEguiState,
    id: egui::Id,
    _commands: &mut Commands,
    _screen_access: &ScreensToImageParam,
    _asset_server: &Res<AssetServer>,
    ui: &mut egui::Ui,
    element_info: &mut ElementInfo,
    _inspector: &mut InspectorUi,
    _popout_pos: &Vec3,
) {
    egui::collapsing_header::CollapsingState::load_with_default_open(
        ui.ctx(),
        id,
        state.expanded.contains(&element_info.drill_id),
    )
    .show_header(ui, |ui| {
        let mut selected = state.selected == Some(element_info.drill_id.clone());
        if ui
            .toggle_value(
                &mut selected,
                format!(
                    "{:?} | {}",
                    element_info.name, element_info.localized_control_type
                ),
            )
            .changed()
        {
            state.selected = if selected {
                Some(element_info.drill_id.clone())
            } else {
                None
            };
        };
    })
    .body(|ui| {
        // if ui.button("Popout").clicked() {
        //     spawn_brick(
        //         commands,
        //         element_info,
        //         element_info.bounding_rect.size(),
        //         *popout_pos,
        //         screen_access,
        //         asset_server,
        //     )
        // }

        if let Some(children) = &mut element_info.children {
            for child in children.iter_mut() {
                ui_for_element_info(
                    state,
                    id.with(child.runtime_id.clone()),
                    _commands,
                    _screen_access,
                    _asset_server,
                    ui,
                    child,
                    _inspector,
                    _popout_pos,
                );
            }
        }
    });
}
