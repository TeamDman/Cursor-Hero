use std::thread;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use cursor_hero_winutils::win_mouse::get_cursor_position;
use uiautomation::UIAutomation;
use uiautomation::UIElement;

use cursor_hero_camera::camera_plugin::MainCamera;

pub struct HoverUiAutomationPlugin;

impl Plugin for HoverUiAutomationPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding HoverInfo resource");
        app.insert_resource(HoverInfo::default())
            .register_type::<Element>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    update_game_mouse_position,
                    update_hover_info,
                    show_hovered_rect,
                )
                    .chain(),
            );
    }
}

#[derive(Debug)]
enum GameboundMessage {
    ScreenHoverInfo(ElementInfo),
    ScreenHoverInfoNone,
    GameHoverInfo(ElementInfo),
    GameHoverInfoNone,
}

#[derive(Debug)]
enum ThreadboundMessage {
    CursorPosition(IVec2),
    CursorPositionNone,
}

#[derive(Resource)]
struct Bridge {
    pub sender: Sender<ThreadboundMessage>,
    pub receiver: Receiver<GameboundMessage>,
}

#[derive(Resource, Reflect, Default)]
pub struct HoverInfo {
    screen_element: Option<ElementInfo>,
    game_element: Option<ElementInfo>,
    enabled: bool,
}
impl HoverInfo {
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.screen_element = None;
            self.game_element = None;
        }
    }
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Debug, Reflect, Clone)]
pub struct ElementInfo {
    pub name: String,
    pub bounding_rect: Rect,
    pub control_type: String,
    pub class_name: String,
    pub automation_id: String,
    // value: Option<String>,
}

#[derive(Component, Reflect)]
pub struct Element {
    pub info: ElementInfo,
}

pub fn get_element_info(element: UIElement) -> Result<ElementInfo, uiautomation::errors::Error> {
    let name = element.get_name()?;
    let bb = element.get_bounding_rectangle()?;
    let class_name = element.get_classname()?;
    let automation_id = element.get_automation_id()?;
    // let value = element.get_property_value().unwrap();

    let info = ElementInfo {
        name,
        bounding_rect: Rect::new(
            bb.get_left() as f32,
            bb.get_top() as f32,
            bb.get_right() as f32,
            bb.get_bottom() as f32,
        ),
        control_type: class_name.clone(),
        class_name,
        automation_id,
        // value,
    };
    Ok(info)
}

fn setup(mut commands: Commands) {
    let (game_tx, game_rx) = bounded::<_>(10);
    let (thread_tx, thread_rx) = bounded::<_>(10);
    commands.insert_resource(Bridge {
        sender: thread_tx,
        receiver: game_rx,
    });

    let game_tx_clone = game_tx.clone();
    thread::Builder::new()
        .name("Screen element hover info thread".to_string())
        .spawn(move || {
            let game_tx = game_tx_clone;
            let automation = UIAutomation::new().unwrap();
            loop {
                if let Ok(cursor_pos) = get_cursor_position() {
                    if let Ok(root) = automation.element_from_point(
                        uiautomation::types::Point::new(cursor_pos.x, cursor_pos.y),
                    ) {
                        let info = get_element_info(root);
                        match info {
                            Ok(info) => {
                                game_tx
                                    .send(GameboundMessage::ScreenHoverInfo(info))
                                    .unwrap();
                            }
                            Err(_) => {
                                game_tx.send(GameboundMessage::ScreenHoverInfoNone).unwrap();
                            }
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        })
        .expect("Failed to spawn screen element hover info thread");

    thread::Builder::new()
        .name("Game element hover info thread".to_string())
        .spawn(move || {
            let automation = UIAutomation::new().unwrap();
            loop {
                // Block until at least one message is available
                let mut msg = thread_rx.recv().unwrap();

                // Check for and use the latest message available
                while let Ok(newer_msg) = thread_rx.try_recv() {
                    msg = newer_msg;
                }
                match msg {
                    ThreadboundMessage::CursorPositionNone => {
                        game_tx.send(GameboundMessage::GameHoverInfoNone).unwrap();
                        continue;
                    }
                    ThreadboundMessage::CursorPosition(cursor_pos) => {
                        if let Ok(root) = automation.element_from_point(
                            uiautomation::types::Point::new(cursor_pos.x, cursor_pos.y),
                        ) {
                            let info = get_element_info(root);
                            match info {
                                Ok(info) => {
                                    game_tx.send(GameboundMessage::GameHoverInfo(info)).unwrap();
                                }
                                Err(_) => {
                                    game_tx.send(GameboundMessage::GameHoverInfoNone).unwrap();
                                }
                            }
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        })
        .expect("Failed to spawn game element hover info thread");
}

fn update_game_mouse_position(
    bridge: ResMut<Bridge>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut debounce: Local<Option<IVec2>>,
    hover_info: Res<HoverInfo>,
) {
    if !hover_info.enabled {
        return;
    }
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    let value = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        .map(|world_position| IVec2::new(world_position.x as i32, -world_position.y as i32));
    if *debounce != value {
        *debounce = value;
        match value {
            Some(value) => bridge
                .sender
                .send(ThreadboundMessage::CursorPosition(value))
                .unwrap(),
            None => bridge
                .sender
                .send(ThreadboundMessage::CursorPositionNone)
                .unwrap(),
        }
    }
}

fn update_hover_info(mut hovered: ResMut<HoverInfo>, bridge: Res<Bridge>) {
    if !hovered.enabled {
        bridge.receiver.try_iter().for_each(drop);
        return;
    }
    if let Ok(msg) = bridge.receiver.try_recv() {
        match msg {
            GameboundMessage::ScreenHoverInfo(info) => {
                hovered.screen_element = Some(info);
            }
            GameboundMessage::ScreenHoverInfoNone => {
                hovered.screen_element = None;
            }
            GameboundMessage::GameHoverInfo(info) => {
                hovered.game_element = Some(info);
            }
            GameboundMessage::GameHoverInfoNone => {
                hovered.game_element = None;
            }
        }
    }
}

#[derive(Component, Reflect, Debug)]
struct ScreenHoveredIndicatorTag;
#[derive(Component, Reflect, Debug)]
struct GameHoveredIndicatorTag;

#[allow(clippy::type_complexity)]
fn show_hovered_rect(
    mut screen_indicator: Query<
        (Entity, &mut Sprite, &mut Transform, &mut Element),
        (
            With<ScreenHoveredIndicatorTag>,
            Without<GameHoveredIndicatorTag>,
        ),
    >,
    mut game_indicator: Query<
        (Entity, &mut Sprite, &mut Transform, &mut Element),
        (
            With<GameHoveredIndicatorTag>,
            Without<ScreenHoveredIndicatorTag>,
        ),
    >,
    hovered: Res<HoverInfo>,
    mut commands: Commands,
) {
    if let Ok((entity, mut sprite, mut transform, mut element)) = screen_indicator.get_single_mut()
    {
        if let Some(info) = &hovered.screen_element {
            sprite.custom_size = Some(Vec2::new(
                info.bounding_rect.width(),
                info.bounding_rect.height(),
            ));
            transform.translation = Vec3::new(
                info.bounding_rect.min.x + info.bounding_rect.width() / 2.,
                -info.bounding_rect.min.y - info.bounding_rect.height() / 2.,
                0.,
            );
            element.info = info.clone();
        } else {
            commands.entity(entity).despawn_recursive();
        }
    } else if let Some(info) = &hovered.screen_element {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    info.bounding_rect.min.x + info.bounding_rect.width() / 2.,
                    -info.bounding_rect.min.y - info.bounding_rect.height() / 2.,
                    0.,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        info.bounding_rect.width(),
                        info.bounding_rect.height(),
                    )),
                    color: Color::rgba(0.141, 0.675, 0.949, 0.05),
                    ..default()
                },
                ..default()
            },
            Name::new("Screen Hovered Indicator"),
            ScreenHoveredIndicatorTag,
            Element { info: info.clone() },
        ));
    }

    if let Ok((entity, mut sprite, mut transform, mut element)) = game_indicator.get_single_mut() {
        if let Some(info) = &hovered.game_element {
            sprite.custom_size = Some(Vec2::new(
                info.bounding_rect.width(),
                info.bounding_rect.height(),
            ));
            transform.translation = Vec3::new(
                info.bounding_rect.min.x + info.bounding_rect.width() / 2.,
                -info.bounding_rect.min.y - info.bounding_rect.height() / 2.,
                0.,
            );
            element.info = info.clone();
        } else {
            commands.entity(entity).despawn_recursive();
        }
    } else if let Some(info) = &hovered.game_element {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    info.bounding_rect.min.x + info.bounding_rect.width() / 2.,
                    -info.bounding_rect.min.y - info.bounding_rect.height() / 2.,
                    0.,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        info.bounding_rect.width(),
                        info.bounding_rect.height(),
                    )),
                    color: Color::rgba(0.641, 0.275, 0.649, 0.05),
                    ..default()
                },
                ..default()
            },
            Name::new("Game Hovered Indicator"),
            GameHoveredIndicatorTag,
            Element { info: info.clone() },
        ));
    }
}
