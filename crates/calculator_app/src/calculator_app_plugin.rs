use std::ops::Neg;

use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use cursor_hero_bevy::prelude::NegativeYVec2;
use cursor_hero_bevy::prelude::NegativeYVec3;
use cursor_hero_bevy::prelude::TranslateVec2;
use cursor_hero_calculator_app_types::calculator_app_types::Calculator;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorElementKind;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorStartMenuPanelButton;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorState;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorTheme;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorThemeKind;
use cursor_hero_calculator_app_types::calculator_app_types::SpawnCalculatorRequestEvent;
use cursor_hero_cursor_types::cursor_click_types::ClickEvent;
use cursor_hero_cursor_types::cursor_click_types::Clickable;
use cursor_hero_cursor_types::cursor_click_types::Way;
use cursor_hero_cursor_types::cursor_hover_types::Hoverable;
use cursor_hero_cursor_types::cursor_types::Cursor;
use cursor_hero_environment_types::environment_types::TrackedEnvironment;
use cursor_hero_start_menu_types::start_menu_types::StartMenuPanel;
use cursor_hero_start_menu_types::start_menu_types::StartMenuPanelAppLauncherIconBuilder;
use cursor_hero_start_menu_types::start_menu_types::StartMenuPanelVisibilityChangeRequestEvent;
use cursor_hero_start_menu_types::start_menu_types::StartMenuPopulateEvent;
use cursor_hero_winutils::win_colors::get_start_color;

pub struct CalculatorAppPlugin;

impl Plugin for CalculatorAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_start_menu_panels);
        app.add_systems(Update, handle_calculator_app_launcher_icon_clicks);
        app.add_systems(Update, handle_spawn_calculator_events);
    }
}

fn populate_start_menu_panels(
    mut commands: Commands,
    mut events: EventReader<StartMenuPopulateEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        if let Err(e) = StartMenuPanelAppLauncherIconBuilder::new(CalculatorStartMenuPanelButton)
            .with_name("Calculator")
            .with_texture(asset_server.load("textures/apps/calculator.png"))
            .spawn(&event.panel, &mut commands)
        {
            error!("Error spawning calculator app launcher icon: {:?}", e);
        }
    }
}

fn handle_calculator_app_launcher_icon_clicks(
    mut start_menu_events: EventWriter<StartMenuPanelVisibilityChangeRequestEvent>,
    mut calculator_events: EventWriter<SpawnCalculatorRequestEvent>,
    mut click_events: EventReader<ClickEvent>,
    cursor_query: Query<&TrackedEnvironment, With<Cursor>>,
    icon_query: Query<&Parent, With<CalculatorStartMenuPanelButton>>,
    panel_query: Query<&Parent, With<StartMenuPanel>>,
) {
    for event in click_events.read() {
        let ClickEvent::Clicked {
            target_id,
            cursor_id,
            way: Way::Left,
        } = event
        else {
            continue;
        };

        let Ok(cursor) = cursor_query.get(*cursor_id) else {
            continue;
        };
        let cursor_environment = cursor;

        let Ok(icon) = icon_query.get(*target_id) else {
            continue;
        };
        let icon_parent = icon;

        let Ok(panel) = panel_query.get(icon_parent.get()) else {
            continue;
        };
        let panel_parent = panel;

        let start_menu_button_id = panel_parent.get();
        start_menu_events.send(StartMenuPanelVisibilityChangeRequestEvent::Close {
            start_menu_button_id,
        });

        let environment_id = cursor_environment.environment_id;
        calculator_events.send(SpawnCalculatorRequestEvent {
            environment_id,
            theme: CalculatorThemeKind::WindowsDark,
            state: CalculatorState {
                expression: "".to_string(),
                value: "0".to_string(),
            },
        });
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
        let size = Vec2::new(320.0, 500.0);
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
                    Calculator,
                    Name::new("Calculator"),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(size),
                            color,
                            ..default()
                        },
                        transform: Transform::from_translation((size / 2.0).extend(1.0).neg_y()),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    let theme = CalculatorThemeKind::WindowsDark;
                    for elem_kind in CalculatorElementKind::variants() {
                        let text = elem_kind
                            .get_text_from_state(&event.state)
                            .unwrap_or_else(|| elem_kind.get_default_text());

                        // convert from top-left offset to center-offset
                        let bounds = theme
                            .get_bounds(&elem_kind)
                            .translated(&(size / 2.0).neg().neg_y());
                        let background_color = theme.get_background_color(&elem_kind);
                        let text_style = theme.get_text_style(&elem_kind);
                        parent
                            .spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(bounds.size()),
                                        color: background_color,
                                        ..default()
                                    },
                                    transform: Transform::from_translation(
                                        bounds.center().extend(2.0),
                                    ),
                                    ..Default::default()
                                },
                                Hoverable,
                                Clickable,
                                RigidBody::Static,
                                Collider::cuboid(bounds.width(), bounds.height()),
                                Name::new(elem_kind.get_name()),
                            ))
                            .with_children(|parent| {
                                parent.spawn(Text2dBundle {
                                    text: Text::from_section(text, text_style),
                                    transform: Transform::from_translation(Vec3::new(
                                        0.0, 0.0, 1.0,
                                    )),
                                    ..default()
                                });
                            });
                    }
                });
        });
    }
}
