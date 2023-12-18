use std::thread;

use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver};
use uiautomation::controls::ControlType;
use uiautomation::types::UIProperty;
use uiautomation::variants::Variant;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;
use windows::{Win32::Foundation::POINT, Win32::UI::WindowsAndMessaging::GetCursorPos};

pub struct HoverUiAutomationPlugin;

impl Plugin for HoverUiAutomationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HoverInfo::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (update_hover_info, show_hovered_rect).chain());
    }
}

#[derive(Resource, Deref)]
struct StreamReceiver(pub Receiver<Option<ElementInfo>>);

#[derive(Resource, Default)]
struct HoverInfo {
    element: Option<ElementInfo>,
}

struct ElementInfo {
    name: String,
    bounding_rect: Rect,
    control_type: String,
    class_name: String,
    automation_id: String,
    // value: Option<String>,
}

fn setup(mut commands: Commands) {
    let (tx, rx) = bounded::<_>(10);
    commands.insert_resource(StreamReceiver(rx));
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
            let cursor_pos = get_cursor_position().expect("Failed to get cursor position");
            // println!("Cursor position: {:?}", cursor_pos);
            if let Ok(root) = automation
                .element_from_point(uiautomation::types::Point::new(cursor_pos.x, cursor_pos.y))
            {
                // print_element(&walker, &root, 0).unwrap();
                let name = root.get_name().unwrap();
                let bb = root.get_bounding_rectangle().unwrap();
                let class_name = root.get_classname().unwrap();
                let automation_id = root.get_automation_id().unwrap();
                // let value = root.get_property_value().unwrap();

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
                tx.send(Some(info)).unwrap();
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });
}

fn update_hover_info(mut hovered: ResMut<HoverInfo>, receiver: Res<StreamReceiver>) {
    if let Ok(data) = receiver.0.try_recv() {
        hovered.element = data;
    }
}

#[derive(Component, Reflect, Debug)]
struct HoveredIndicatorTag;

fn show_hovered_rect(
    mut indicator: Query<(Entity, &mut Sprite, &mut Transform), With<HoveredIndicatorTag>>,
    hovered: Res<HoverInfo>,
    mut commands: Commands,
) {
    if let Ok((entity, mut sprite, mut transform)) = indicator.get_single_mut() {
        if let Some(info) = &hovered.element {
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
    } else if let Some(info) = &hovered.element {
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
            HoveredIndicatorTag,
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
