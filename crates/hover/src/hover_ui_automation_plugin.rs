use std::thread;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use cursor_hero_ui_automation::prelude::find_element_at;
use cursor_hero_ui_automation::prelude::gather_single_element_info;
use cursor_hero_ui_automation::prelude::ElementInfo;
use cursor_hero_winutils::win_mouse::get_cursor_position;

use cursor_hero_camera::camera_plugin::MainCamera;

pub struct HoverUiAutomationPlugin;

impl Plugin for HoverUiAutomationPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding HoverInfo resource");
        app.insert_resource(HoverInfo::default());
        app.register_type::<HoveredElement>();
        app.add_systems(Startup, setup);
        app.add_systems(
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

#[derive(Resource, Default)]
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

#[derive(Component, Reflect)]
pub struct HoveredElement {
    pub info: ElementInfo,
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
            loop {
                if let Ok(cursor_pos) = get_cursor_position() {
                    if let Ok(root) = find_element_at(cursor_pos) {
                        let info = gather_single_element_info(&root);
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
            loop {
                // Block until at least one message is available
                let mut msg = match thread_rx.recv() {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!("Failed to receive thread message, exiting: {:?}", e);
                        break;
                    }
                };

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
                        if let Ok(root) = find_element_at(cursor_pos) {
                            let info = gather_single_element_info(&root);
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
        (Entity, &mut Sprite, &mut Transform, &mut HoveredElement),
        (
            With<ScreenHoveredIndicatorTag>,
            Without<GameHoveredIndicatorTag>,
        ),
    >,
    mut game_indicator: Query<
        (Entity, &mut Sprite, &mut Transform, &mut HoveredElement),
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
            let bounds = info.bounding_rect.as_rect();
            sprite.custom_size = Some(Vec2::new(
                bounds.width(),
                bounds.height(),
            ));
            transform.translation = Vec3::new(
                bounds.min.x + bounds.width() / 2.,
                -bounds.min.y - bounds.height() / 2.,
                0.,
            );
            element.info = info.clone();
        } else {
            commands.entity(entity).despawn_recursive();
        }
    } else if let Some(info) = &hovered.screen_element {
        let bounds = info.bounding_rect.as_rect();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    bounds.min.x + bounds.width() / 2.,
                    -bounds.min.y - bounds.height() / 2.,
                    0.,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        bounds.width(),
                        bounds.height(),
                    )),
                    color: Color::rgba(0.141, 0.675, 0.949, 0.05),
                    ..default()
                },
                ..default()
            },
            Name::new("Screen Hovered Indicator"),
            ScreenHoveredIndicatorTag,
            HoveredElement { info: info.clone() },
        ));
    }

    if let Ok((entity, mut sprite, mut transform, mut element)) = game_indicator.get_single_mut() {
        if let Some(info) = &hovered.game_element {
            let bounds = info.bounding_rect.as_rect();
            sprite.custom_size = Some(Vec2::new(
                bounds.width(),
                bounds.height(),
            ));
            transform.translation = Vec3::new(
                bounds.min.x + bounds.width() / 2.,
                -bounds.min.y - bounds.height() / 2.,
                0.,
            );
            element.info = info.clone();
        } else {
            commands.entity(entity).despawn_recursive();
        }
    } else if let Some(info) = &hovered.game_element {
        let bounds = info.bounding_rect.as_rect();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    bounds.min.x + bounds.width() / 2.,
                    -bounds.min.y - bounds.height() / 2.,
                    0.,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        bounds.width(),
                        bounds.height(),
                    )),
                    color: Color::rgba(0.641, 0.275, 0.649, 0.05),
                    ..default()
                },
                ..default()
            },
            Name::new("Game Hovered Indicator"),
            GameHoveredIndicatorTag,
            HoveredElement { info: info.clone() },
        ));
    }
}
