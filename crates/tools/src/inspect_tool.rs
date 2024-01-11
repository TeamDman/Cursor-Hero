use std::thread;

use bevy::prelude::*;
use image::DynamicImage;
use image::RgbImage;

use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use crossbeam_channel::Receiver;
use cursor_hero_hover::hover_ui_automation_plugin::get_element_info;
use cursor_hero_physics::damping_plugin::MovementDamping;
use cursor_hero_screen::screen_plugin::Screen;
use leafwing_input_manager::prelude::*;

use crossbeam_channel::bounded;
use crossbeam_channel::Sender;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_hover::hover_ui_automation_plugin::ElementInfo;
use cursor_hero_pointer::pointer_plugin::Pointer;
use cursor_hero_winutils::win_mouse::find_element_at;

use cursor_hero_toolbelt::types::*;

use crate::cube_tool::CubeToolInteractable;
use crate::prelude::*;

pub struct InspectToolPlugin;

impl Plugin for InspectToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InspectTool>()
            .add_plugins(InputManagerPlugin::<InspectToolAction>::default())
            .add_systems(Startup, spawn_worker_thread)
            .add_systems(Update, (toolbelt_events, handle_input, handle_replies));
    }
}

#[derive(Component, Reflect)]
struct InspectTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            ToolbeltEvent::PopulateInspectorToolbelt(toolbelt_id) => {
                spawn_action_tool!(
                    e,
                    &mut commands,
                    *toolbelt_id,
                    &asset_server,
                    InspectTool,
                    InspectToolAction
                );
            }
            _ => {}
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum InspectToolAction {
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
}
impl ToolAction for InspectToolAction {
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

fn handle_input(
    tools: Query<(&ActionState<InspectToolAction>, Option<&ToolActiveTag>, &Parent)>,
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

fn handle_replies(
    mut commands: Commands,
    bridge: Res<Bridge>,
    mut images: ResMut<Assets<Image>>,
    screens: Query<(&Handle<Image>, &GlobalTransform), With<Screen>>,
) {
    while let Ok(msg) = bridge.receiver.try_recv() {
        match msg {
            GameboundMessage::ElementDetails(info) => {
                info!("Received element name: {}", info.name);
                let elem_rect = info.bounding_rect;
                println!("elem_rect: {:?}", elem_rect);
                let elem_size = info.bounding_rect.max - info.bounding_rect.min;
                let mut tex = RgbImage::new(elem_size.x as u32, elem_size.y as u32);

                // find out what parts of each screen are intersecting with the element
                for (screen_image_handle, screen_trans) in screens.iter() {
                    // find out the image size
                    let screen_center_pos = screen_trans.translation();
                    match images.get(screen_image_handle) {
                        None => {}
                        Some(screen_image) => {
                            // Calculate the overlapping area
                            let screen_size = screen_image.texture_descriptor.size;
                            let mut screen_origin = screen_center_pos.xy();
                            screen_origin.y *= -1.0;
                            let screen_rect = Rect::from_center_size(
                                screen_origin,
                                Vec2::new(screen_size.width as f32, screen_size.height as f32),
                            );

                            // find the overlap
                            println!("screen_rect: {:?}", screen_rect);
                            let intersection = screen_rect.intersect(elem_rect);
                            println!("intersection rect: {:?}", intersection);

                            // convert to monitor coordinates
                            let origin = intersection.center() - screen_rect.min.xy();
                            let tex_grab_rect = Rect::from_center_size(origin, intersection.size());
                            println!("tex_grab_rect: {:?}", tex_grab_rect);

                            if !tex_grab_rect.is_empty() {
                                println!(
                                    "Copying pixel range {} by {}",
                                    tex_grab_rect.size().x,
                                    tex_grab_rect.size().y
                                );

                                // Calculate where to start placing pixels in the element's texture
                                let texture_start_x = (intersection.min.x - elem_rect.min.x) as u32;
                                let texture_start_y = (intersection.min.y - elem_rect.min.y) as u32;
                                println!("Texture start: {} {}", texture_start_x, texture_start_y);
                                // Copy the overlapping part of the screen texture to the element's texture.
                                for y in tex_grab_rect.min.y as usize..tex_grab_rect.max.y as usize
                                {
                                    for x in
                                        tex_grab_rect.min.x as usize..tex_grab_rect.max.x as usize
                                    {
                                        let start = (y * screen_size.width as usize + x) * 4;
                                        if start + 4 <= screen_image.data.len() {
                                            let pixel: [u8; 3] = [
                                                screen_image.data[start],
                                                screen_image.data[start + 1],
                                                screen_image.data[start + 2],
                                                // screen_image.data[start + 3],
                                            ];
                                            tex.put_pixel(
                                                texture_start_x + x as u32
                                                    - tex_grab_rect.min.x as u32,
                                                texture_start_y + y as u32
                                                    - tex_grab_rect.min.y as u32,
                                                image::Rgb(pixel),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                let dynamic_image = DynamicImage::ImageRgb8(tex);
                let image = Image::from_dynamic(dynamic_image, true);
                let texture_handle = images.add(image);

                // spawn the element image
                let mut elem_center_pos = (info.bounding_rect.min + elem_size / 2.0).extend(20.0);
                elem_center_pos.y *= -1.0;
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(elem_center_pos),
                        sprite: Sprite {
                            custom_size: Some(elem_size),
                            // color: Color::PURPLE,
                            ..default()
                        },
                        texture: texture_handle,
                        ..default()
                    },
                    CubeToolInteractable,
                    RigidBody::Dynamic,
                    Collider::cuboid(elem_size.x, elem_size.y),
                    MovementDamping::default(),
                    Name::new(format!("Element - {}", info.name)),
                ));
            }
        }
    }
}
