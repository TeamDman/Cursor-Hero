use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use cursor_hero_bevy::prelude::NegativeYVec2;
use cursor_hero_bevy::prelude::TranslateVec2;
use cursor_hero_explorer_app_types::prelude::Explorer;
use cursor_hero_explorer_app_types::prelude::ExplorerElementKind;
use cursor_hero_explorer_app_types::prelude::ExplorerTheme;
use cursor_hero_explorer_app_types::prelude::ExplorerThemeKind;
use cursor_hero_explorer_app_types::prelude::SpawnExplorerRequestEvent;
use cursor_hero_cursor_types::cursor_click_types::Clickable;
use cursor_hero_cursor_types::cursor_hover_types::Hoverable;
use cursor_hero_winutils::win_colors::get_start_color;
use std::ops::Neg;

pub struct ExplorerSpawningPlugin;

impl Plugin for ExplorerSpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_spawn_explorer_events);
    }
}

fn handle_spawn_explorer_events(
    mut commands: Commands,
    mut events: EventReader<SpawnExplorerRequestEvent>,
) {
    for event in events.read() {
        let SpawnExplorerRequestEvent { environment_id, .. } = event;
        let Some(mut environment) = commands.get_entity(*environment_id) else {
            warn!("Couldn't find environment when processing {:?}", event);
            continue;
        };
        let border = 4.0;
        let size = event
            .theme
            .get_bounds(&ExplorerElementKind::Background)
            .size()
            + border * 2.0;
        let color = match get_start_color() {
            Ok(color) => color,
            Err(err) => {
                warn!("Couldn't get accent color: {:?}", err);
                Color::rgba(0.0, 0.0, 0.0, 1.0)
            }
        };

        environment.with_children(|parent| {
            parent
                .spawn((
                    Explorer,
                    Name::new("Explorer"),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(size),
                            color,
                            ..default()
                        },
                        transform: Transform::from_translation(event.position.extend(1.0)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    let theme = ExplorerThemeKind::WindowsDark;
                    for elem_kind in ExplorerElementKind::variants() {
                        let text = elem_kind
                            .get_text_from_state(&event.state)
                            .unwrap_or_else(|| elem_kind.get_default_text());

                        // convert from top-left offset to center-offset
                        let bounds = theme
                            .get_bounds(&elem_kind)
                            .translated(&(size / 2.0).neg().neg_y());
                        let background_color = theme.get_background_color(&elem_kind);
                        let text_style = theme.get_text_style(&elem_kind);
                        let mut elem_ent = parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(bounds.size()),
                                    color: background_color,
                                    ..default()
                                },
                                transform: Transform::from_translation(
                                    (bounds.center() + Vec2::new(border, -border)).extend(2.0),
                                ),
                                ..Default::default()
                            },
                            Name::new(elem_kind.get_name()),
                        ));
                        if elem_kind != ExplorerElementKind::Background {
                            elem_ent.insert((
                                Hoverable,
                                Clickable,
                                RigidBody::Static,
                                Collider::cuboid(bounds.width(), bounds.height()),
                            ));
                        }

                        elem_ent.with_children(|parent| {
                            parent.spawn(Text2dBundle {
                                text: Text::from_section(text, text_style),
                                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                                ..default()
                            });
                        });
                    }
                });
        });
    }
}
