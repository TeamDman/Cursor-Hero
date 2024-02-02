use bevy::prelude::*;
use cursor_hero_winutils::win_colors::get_accent_color;

use crate::start_menu_button_plugin::StartMenuButton;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StartMenu>();
        app.add_event::<StartMenuEvent>();
        app.add_systems(Update, handle_open_events);
        app.add_systems(Update, handle_close_events);
    }
}

#[derive(Component, Debug, Reflect)]
pub struct StartMenu;

#[derive(Event, Debug, Reflect)]
pub enum StartMenuEvent {
    Open { start_menu_button_id: Entity },
    Close { start_menu_button_id: Entity },
}

fn handle_open_events(
    mut commands: Commands,
    mut start_menu_events: EventReader<StartMenuEvent>,
    start_menu_button_query: Query<&Sprite, With<StartMenuButton>>,
) {
    for event in start_menu_events.read() {
        let StartMenuEvent::Open {
            start_menu_button_id,
        } = event
        else {
            continue;
        };
        info!("Opening start menu for button {:?}", start_menu_button_id);
        let Ok(Sprite {
            custom_size: Some(start_menu_button_size),
            ..
        }) = start_menu_button_query.get(*start_menu_button_id)
        else {
            warn!(
                "Couldn't find start menu button sprite for {:?}",
                start_menu_button_id
            );
            continue;
        };
        let size = Vec2::new(400.0, 400.0);
        let start_menu_pos = ((size / 2.0)
            + Vec2::new(
                -start_menu_button_size.x / 2.0,
                start_menu_button_size.y / 2.0,
            ))
        .extend(3.0);
        commands
            .entity(*start_menu_button_id)
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(size),
                            color: match get_accent_color() {
                                Ok(color) => color,
                                Err(err) => {
                                    warn!("Couldn't get start color: {}", err);
                                    Color::rgb(0.0, 0.0, 0.0)
                                }
                            },
                            ..default()
                        },
                        transform: Transform::from_translation(start_menu_pos),
                        ..Default::default()
                    },
                    StartMenu,
                ));
            });
    }
}

fn handle_close_events(
    mut commands: Commands,
    mut start_menu_events: EventReader<StartMenuEvent>,
    start_menu_button_query: Query<&Children, With<StartMenuButton>>,
    start_menu_query: Query<(), With<StartMenu>>,
) {
    for event in start_menu_events.read() {
        let StartMenuEvent::Close {
            start_menu_button_id,
        } = event
        else {
            continue;
        };
        info!("Closing start menu for button {:?}", start_menu_button_id);
        let Ok(start_menu_button_children) = start_menu_button_query.get(*start_menu_button_id)
        else {
            warn!(
                "Couldn't find start menu button children for {:?}",
                start_menu_button_id
            );
            continue;
        };
        let mut removed = vec![];
        for child in start_menu_button_children.iter() {
            if start_menu_query.get(*child).is_ok() {
                commands.entity(*child).despawn_recursive();
                removed.push(*child);
            }
        }
        commands
            .entity(*start_menu_button_id)
            .remove_children(&removed);
    }
}
