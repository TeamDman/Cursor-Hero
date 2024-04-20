use bevy::prelude::*;
use cursor_hero_bevy::prelude::NegativeYVec3;
use cursor_hero_calculator_app_types::calculator_app_types::Calculator;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorButton;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorDisplay;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorExpression;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorStartMenuPanelButton;
use cursor_hero_calculator_app_types::calculator_app_types::SpawnCalculatorRequestEvent;
use cursor_hero_cursor_types::cursor_click_types::ClickEvent;
use cursor_hero_cursor_types::cursor_click_types::Way;
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
            display: "".to_string(),
            expression: "".to_string(),
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
                        transform: Transform::from_translation((size/2.0).extend(1.0).neg_y()),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        CalculatorExpression,
                        Text2dBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    ..default()
                                }],
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        CalculatorDisplay,
                        Text2dBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    ..default()
                                }],
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(0.0, 50.0, 2.0)),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        CalculatorButton,
                        Text2dBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "=".to_string(),
                                    ..default()
                                }],
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 3.0)),
                            ..default()
                        },
                    ));
                });
        });
    }
}
