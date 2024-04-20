use bevy::prelude::*;

use cursor_hero_start_menu_types::prelude::*;
use cursor_hero_winutils::win_colors::get_accent_color;
pub struct StartMenuPanelPlugin;

impl Plugin for StartMenuPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_open_events);
        app.add_systems(Update, handle_close_events);
    }
}

fn handle_open_events(
    mut commands: Commands,
    mut open_events: EventReader<StartMenuPanelVisibilityChangeRequestEvent>,
    mut populate_events: EventWriter<StartMenuPopulateEvent>,
    start_menu_button_query: Query<&Sprite, With<StartMenuButton>>,
) {
    for event in open_events.read() {
        let StartMenuPanelVisibilityChangeRequestEvent::Open {
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
                let panel = parent
                    .spawn((
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
                        StartMenuPanel,
                    ))
                    .id();
                populate_events.send(StartMenuPopulateEvent {
                    panel,
                    button: *start_menu_button_id,
                });
            });
    }
}

fn handle_close_events(
    mut commands: Commands,
    mut events: EventReader<StartMenuPanelVisibilityChangeRequestEvent>,
    button_query: Query<&Children, With<StartMenuButton>>,
    panel_query: Query<(), With<StartMenuPanel>>,
) {
    for event in events.read() {
        let StartMenuPanelVisibilityChangeRequestEvent::Close {
            start_menu_button_id,
        } = event
        else {
            continue;
        };
        info!("Closing start menu for button {:?}", start_menu_button_id);
        let Ok(children) = button_query.get(*start_menu_button_id) else {
            warn!(
                "Couldn't find start menu button children for {:?}",
                start_menu_button_id
            );
            continue;
        };
        let mut removed = vec![];
        for child in children.iter() {
            if panel_query.get(*child).is_ok() {
                commands.entity(*child).despawn_recursive();
                removed.push(*child);
            }
        }
        commands
            .entity(*start_menu_button_id)
            .remove_children(&removed);
    }
}
