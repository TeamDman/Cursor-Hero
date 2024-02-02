use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use bevy_xpbd_2d::components::Sensor;
use cursor_hero_pointer_types::prelude::*;

use crate::game_screen_taskbar_plugin::Taskbar;
use crate::start_menu_plugin::StartMenu;
use crate::start_menu_plugin::StartMenuEvent;

pub struct StartMenuButtonPlugin;

impl Plugin for StartMenuButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StartMenuButton>();
        app.add_systems(Update, add_start_menu_button_to_new_taskbars);
        app.add_systems(Update, click_listener);
        app.add_systems(Update, visuals);
    }
}

#[derive(Component, Debug, Reflect)]
pub struct StartMenuButton;

fn add_start_menu_button_to_new_taskbars(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    taskbar_query: Query<(Entity, &Sprite), Added<Taskbar>>,
) {
    for (taskbar_id, taskbar_sprite) in taskbar_query.iter() {
        let Some(taskbar_size) = taskbar_sprite.custom_size else {
            warn!("Taskbar {:?} has no custom size", taskbar_id);
            continue;
        };
        let start_menu_button_size = Vec2::new(48.0, 40.0);
        let start_menu_button_translation = Vec3::new(
            -taskbar_size.x / 2.0 + start_menu_button_size.x / 2.0,
            0.0,
            1.0,
        );
        info!("Adding start menu button to taskbar {:?}", taskbar_id);
        commands.entity(taskbar_id).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(start_menu_button_size),
                        ..default()
                    },
                    texture: asset_server.load("textures/environment/game/start_menu_button.png"),
                    transform: Transform::from_translation(start_menu_button_translation),
                    ..Default::default()
                },
                RigidBody::Static,
                Collider::cuboid(start_menu_button_size.x, start_menu_button_size.y),
                Sensor,
                Name::new("Start Menu Button"),
                StartMenuButton,
                Hoverable,
                Clickable,
            ));
        });
    }
}

fn click_listener(
    mut click_events: EventReader<ClickEvent>,
    start_menu_button_query: Query<&Children, With<StartMenuButton>>,
    start_menu_query: Query<(), With<StartMenu>>,
    mut start_menu_events: EventWriter<StartMenuEvent>,
) {
    for event in click_events.read() {
        let ClickEvent::Clicked {
            target_id,
            pointer_id: _,
            way,
        } = event
        else {
            continue;
        };
        if way != &Way::Left {
            continue;
        }
        if let Ok(children) = start_menu_button_query.get(*target_id) {
            info!("Start menu button clicked");
            let open = children
                .iter()
                .any(|child| start_menu_query.get(*child).is_ok());
            if open {
                start_menu_events.send(StartMenuEvent::Close {
                    start_menu_button_id: *target_id,
                });
            } else {
                start_menu_events.send(StartMenuEvent::Open {
                    start_menu_button_id: *target_id,
                });
            }
        }
    }
}

enum VisualState {
    Normal,
    Hovered,
    Pressed,
}

#[allow(clippy::type_complexity)]
fn visuals(
    mut start_menu_button_query: Query<
        (&mut Sprite, Option<&Pressed>, Option<&Hovered>),
        With<StartMenuButton>,
    >,
) {
    for (mut sprite, pressed, hovered) in start_menu_button_query.iter_mut() {
        let mut visual_state = VisualState::Normal;
        if pressed.is_some() {
            visual_state = VisualState::Pressed;
        } else if hovered.is_some() {
            visual_state = VisualState::Hovered;
        }
        match visual_state {
            VisualState::Normal => {
                sprite.color = Color::WHITE;
            }
            VisualState::Hovered => {
                sprite.color = Color::ORANGE_RED;
            }
            VisualState::Pressed => {
                sprite.color = Color::RED;
            }
        }
    }
}
