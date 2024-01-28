use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use bevy_xpbd_2d::components::Sensor;
use cursor_hero_pointer::pointer_click_plugin::ClickEvent;
use cursor_hero_pointer::pointer_click_plugin::Clickable;
use cursor_hero_pointer::pointer_hover_plugin::Hoverable;

use crate::game_screen_taskbar_plugin::Taskbar;

pub struct StartMenuButtonPlugin;

impl Plugin for StartMenuButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StartMenuButton>();
        app.add_systems(Update, add_start_menu_button_to_new_taskbars);
        app.add_systems(Update, click_listener);
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
        let start_menu_button_translation = Vec3::new(-taskbar_size.x / 2.0 + 40.0 / 2.0, 0.0, 1.0);
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

fn click_listener(mut click_events: EventReader<ClickEvent>) {
    for event in click_events.read() {
        debug!("Click event: {:?}", event);
    }
}
