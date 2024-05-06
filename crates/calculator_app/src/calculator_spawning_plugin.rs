use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use cursor_hero_bevy::prelude::NegativeYVec2;
use cursor_hero_bevy::prelude::TranslateVec2;
use cursor_hero_calculator_app_types::calculator_app_types::Calculator;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorElementKind;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorTheme;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorThemeKind;
use cursor_hero_calculator_app_types::calculator_app_types::SpawnCalculatorRequestEvent;
use cursor_hero_cursor_types::cursor_click_types::Clickable;
use cursor_hero_cursor_types::cursor_hover_types::Hoverable;
use cursor_hero_environment_types::environment_types::TrackedEnvironment;
use cursor_hero_winutils::win_colors::get_start_color;
use std::ops::Neg;

pub struct CalculatorSpawningPlugin;

impl Plugin for CalculatorSpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_spawn_calculator_events);
    }
}

fn handle_spawn_calculator_events(
    mut commands: Commands,
    mut events: EventReader<SpawnCalculatorRequestEvent>,
) {
    for event in events.read() {
        let SpawnCalculatorRequestEvent { environment_id, .. } = event;
        let Some(mut environment) = commands.get_entity(*environment_id) else {
            warn!("Couldn't find environment when processing {:?}", event);
            continue;
        };
        let border = 4.0;
        let size = event
            .theme
            .get_bounds(&CalculatorElementKind::Background)
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
                    Calculator::default(),
                    TrackedEnvironment {
                        environment_id: *environment_id
                    },
                    Name::new("Calculator"),
                    // SpatialBundle {
                    //     transform: Transform::from_translation(Vec3::ZERO),
                    //     ..default()
                    // },
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
                    let theme = CalculatorThemeKind::WindowsDark;
                    for elem_kind in CalculatorElementKind::variants() {
                        let text = elem_kind
                            .get_text_from_state(&event.state)
                            .or_else(|| elem_kind.get_default_text());

                        // convert from top-left offset to center-offset
                        let bounds = theme
                            .get_bounds(&elem_kind)
                            .translated(&(size / 2.0).neg().neg_y());
                        let background_color = theme.get_background_color(&elem_kind);
                        let text_style = theme.get_text_style(&elem_kind);
                        let z = theme.get_z_offset(&elem_kind);
                        let mut elem_ent = parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(bounds.size()),
                                    color: background_color,
                                    ..default()
                                },
                                transform: Transform::from_translation(
                                    (bounds.center() + Vec2::new(border, -border)).extend(1.0 + z),
                                ),
                                ..Default::default()
                            },
                            Name::new(elem_kind.get_name()),
                        ));
                        elem_kind.populate(&mut elem_ent);
                        if elem_kind != CalculatorElementKind::Background {
                            elem_ent.insert((
                                Hoverable,
                                Clickable,
                                RigidBody::Static,
                                Collider::cuboid(bounds.width(), bounds.height()),
                            ));
                        }
                        if let Some(text) = text {
                            elem_ent.with_children(|parent| {
                                parent.spawn(Text2dBundle {
                                    text: Text::from_section(text, text_style),
                                    transform: Transform::from_translation(Vec3::new(
                                        0.0, 0.0, 1.0,
                                    )),
                                    ..default()
                                });
                            });
                        }
                    }
                });
        });
    }
}
