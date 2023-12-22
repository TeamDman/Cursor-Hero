use std::thread;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crossbeam_channel::Sender;
use crossbeam_channel::{bounded, Receiver};
use uiautomation::controls::ControlType;
use uiautomation::types::UIProperty;
use uiautomation::variants::Variant;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
// use uiautomation::UITreeWalker;
use windows::{Win32::Foundation::POINT, Win32::UI::WindowsAndMessaging::GetCursorPos};

use super::camera_plugin::MainCamera;

pub struct HoverUiAutomationPlugin;

impl Plugin for HoverUiAutomationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HoverInfo::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    update_game_mouse_position,
                    update_screen_hover_info,
                    update_game_hover_info,
                    show_hovered_rect,
                )
                    .chain(),
            );
    }
}

#[derive(Resource, Deref)]
struct ScreenHoveredBridge(pub Receiver<Option<ElementInfo>>);
#[derive(Resource)]
struct GameHoveredBridge {
    debounce: Option<(isize, isize)>,
    pub sender: Sender<Option<(isize, isize)>>,
    pub receiver: Receiver<Option<ElementInfo>>,
}

#[derive(Resource, Default)]
struct HoverInfo {
    screen_element: Option<ElementInfo>,
    game_element: Option<ElementInfo>,
}

struct ElementInfo {
    name: String,
    bounding_rect: Rect,
    control_type: String,
    class_name: String,
    automation_id: String,
    // value: Option<String>,
}

fn get_element_info(element: UIElement) -> Result<ElementInfo, uiautomation::errors::Error> {
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
    return Ok(info);
}

fn setup(mut commands: Commands) {
    let (tx, rx) = bounded::<_>(10);
    commands.insert_resource(ScreenHoveredBridge(rx));
    thread::spawn(move || {
        let automation = UIAutomation::new().unwrap();
        let filter = automation
            .create_property_condition(
                UIProperty::ControlType,
                Variant::from(ControlType::Pane as i32),
                None,
            )
            .unwrap();
        // let walker = automation.filter_tree_walker(filter).unwrap(); //automation.get_control_view_walker().unwrap();

        loop {
            if let Ok(cursor_pos) = get_cursor_position() {
                // println!("Cursor position: {:?}", cursor_pos);
                if let Ok(root) = automation
                    .element_from_point(uiautomation::types::Point::new(cursor_pos.x, cursor_pos.y))
                {
                    let info = get_element_info(root);
                    tx.send(info.ok()).unwrap();
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    let (game_tx, game_rx) = bounded::<_>(10);
    let (thread_tx, thread_rx) = bounded::<_>(10);
    commands.insert_resource(GameHoveredBridge {
        debounce: None,
        sender: thread_tx,
        receiver: game_rx,
    });
    thread::spawn(move || {
        let automation = UIAutomation::new().unwrap();
        let filter = automation
            .create_property_condition(
                UIProperty::ControlType,
                Variant::from(ControlType::Pane as i32),
                None,
            )
            .unwrap();
        // let walker = automation.filter_tree_walker(filter).unwrap(); //automation.get_control_view_walker().unwrap();

        loop {
            // Block until at least one message is available
            let mut latest_position = thread_rx.recv().unwrap();

            // Check for and use the latest message available
            while let Ok(newer_position) = thread_rx.try_recv() {
                latest_position = newer_position;
            }
            let cursor_pos = latest_position;
            if cursor_pos.is_none() {
                game_tx.send(None).unwrap();
                continue;
            }
            let cursor_pos = cursor_pos.unwrap();
            if let Ok(root) = automation.element_from_point(uiautomation::types::Point::new(
                cursor_pos.0 as i32,
                cursor_pos.1 as i32,
            )) {
                let info = get_element_info(root);
                game_tx.send(info.ok()).unwrap();
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });
}

fn update_game_mouse_position(
    mut bridge: ResMut<GameHoveredBridge>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    let value = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        .map(|world_position| (world_position.x as isize, -world_position.y as isize));
    if bridge.debounce != value {
        bridge.debounce = value;
        bridge.sender.send(value).unwrap();
    }
}

fn update_screen_hover_info(mut hovered: ResMut<HoverInfo>, receiver: Res<ScreenHoveredBridge>) {
    if let Ok(data) = receiver.0.try_recv() {
        hovered.screen_element = data;
    }
}
fn update_game_hover_info(mut hovered: ResMut<HoverInfo>, bridge: Res<GameHoveredBridge>) {
    if let Ok(data) = bridge.receiver.try_recv() {
        hovered.game_element = data;
    }
}

#[derive(Component, Reflect, Debug)]
struct ScreenHoveredIndicatorTag;
#[derive(Component, Reflect, Debug)]
struct GameHoveredIndicatorTag;

fn show_hovered_rect(
    mut screen_indicator: Query<
        (Entity, &mut Sprite, &mut Transform),
        (
            With<ScreenHoveredIndicatorTag>,
            Without<GameHoveredIndicatorTag>,
        ),
    >,
    mut game_indicator: Query<
        (Entity, &mut Sprite, &mut Transform),
        (
            With<GameHoveredIndicatorTag>,
            Without<ScreenHoveredIndicatorTag>,
        ),
    >,
    hovered: Res<HoverInfo>,
    mut commands: Commands,
) {
    if let Ok((entity, mut sprite, mut transform)) = screen_indicator.get_single_mut() {
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
        ));
    }

    if let Ok((entity, mut sprite, mut transform)) = game_indicator.get_single_mut() {
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
        ));
    }
}

fn get_cursor_position() -> Result<POINT, windows::core::Error> {
    unsafe {
        let mut point = POINT::default();
        GetCursorPos(&mut point)?;
        Ok(point)
    }
}
