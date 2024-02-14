use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_environment_types::prelude::*;
use cursor_hero_math::Lerp;
use cursor_hero_ollama_types::prelude::*;
use cursor_hero_pointer_types::prelude::*;
pub struct OllamaButtonPlugin;

impl Plugin for OllamaButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_new_host_environments);
        app.add_systems(Update, update_visuals);
        app.add_systems(Update, status_click_listener);
        app.add_systems(Update, terminal_click_listener);
        app.add_systems(Update, handle_terminal_events);
    }
}

fn populate_new_host_environments(
    mut commands: Commands,
    mut environment_events: EventReader<PopulateEnvironmentEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in environment_events.read() {
        let PopulateEnvironmentEvent::Host { environment_id } = event else {
            continue;
        };
        info!(
            "Adding Ollama Server Control to new host environment {:?}",
            environment_id
        );
        commands.entity(*environment_id).with_children(|parent| {
            parent
                .spawn((
                    OllamaStatusButton::default(),
                    Name::new("Ollama Server Control"),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(200.0, 100.0)),
                            color: Color::PURPLE,
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            1920.0 / 2.0,
                            -1080.0 - 200.0,
                            0.0,
                        )),
                        ..default()
                    },
                    Clickable,
                    Hoverable,
                    RigidBody::Static,
                    Sensor,
                    Collider::cuboid(200.0, 100.0),
                ))
                .with_children(|parent| {
                    parent.spawn((Text2dBundle {
                        text: Text::from_section(
                            "Ollama Server Control".to_string(),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        transform: Transform::from_xyz(0.0, 70.0, 1.0),
                        ..default()
                    },));
                });
            parent
                .spawn((
                    OllamaTerminalButton::default(),
                    Name::new("Ollama Terminal Button"),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(200.0, 100.0)),
                            color: Color::rgb(0.0, 0.6, 0.8),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            1920.0 / 2.0,
                            -1080.0 - 350.0,
                            0.0,
                        )),
                        ..default()
                    },
                    Clickable,
                    Hoverable,
                    RigidBody::Static,
                    Sensor,
                    Collider::cuboid(200.0, 100.0),
                ))
                .with_children(|parent| {
                    parent.spawn((Text2dBundle {
                        text: Text::from_section(
                            "open terminal".to_string(),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..default()
                    },));
                });
        });
    }
}

fn update_visuals(
    mut events: EventReader<OllamaStatusEvent>,
    mut button_query: Query<(&mut Sprite, &Children, &mut OllamaStatusButton)>,
    mut button_text_query: Query<&mut Text>,
) {
    for event in events.read() {
        let OllamaStatusEvent::Changed { new_value: status } = event else {
            continue;
        };
        debug!("Updating Ollama Server Control visuals to {:?}", status);
        for button in button_query.iter_mut() {
            let (mut button_sprite, button_children, mut button) = button;
            button.visual_state = match button.visual_state {
                OllamaStatusButtonVisualState::Default { .. } => {
                    OllamaStatusButtonVisualState::Default { status: *status }
                }
                OllamaStatusButtonVisualState::Hovered { .. } => {
                    OllamaStatusButtonVisualState::Hovered { status: *status }
                }
                OllamaStatusButtonVisualState::Pressed { .. } => {
                    OllamaStatusButtonVisualState::Pressed { status: *status }
                }
            };
            match status {
                OllamaStatus::Alive => {
                    button_sprite.color = Color::GREEN;
                }
                OllamaStatus::Dead => {
                    button_sprite.color = Color::RED;
                }
                OllamaStatus::Unknown => {
                    button_sprite.color = Color::PURPLE;
                }
                OllamaStatus::Starting { instant, timeout } => {
                    button_sprite.color = Color::YELLOW
                        * (1.0, 0.1).lerp(instant.elapsed().as_secs_f32() / timeout.as_secs_f32());
                }
            }
            for child in button_children.iter() {
                if let Ok(mut text) = button_text_query.get_mut(*child) {
                    match status {
                        OllamaStatus::Alive => {
                            text.sections[0].value = "Ollama Server Control (Alive)".to_string();
                        }
                        OllamaStatus::Dead => {
                            text.sections[0].value = "Ollama Server Control (Dead)".to_string();
                        }
                        OllamaStatus::Unknown => {
                            text.sections[0].value = "Ollama Server Control (Unknown)".to_string();
                        }
                        OllamaStatus::Starting { instant, .. } => {
                            text.sections[0].value = format!(
                                "Ollama Server Control (Starting {}s ago)",
                                instant.elapsed().as_secs()
                            );
                        }
                    }
                }
            }
        }
    }

    for button in button_query.iter_mut() {
        let (mut sprite, children, button) = button;
        // if the visual state status is starting, update the text to show the time elapsed
        let (OllamaStatusButtonVisualState::Default {
            status: OllamaStatus::Starting { instant, timeout },
        }
        | OllamaStatusButtonVisualState::Hovered {
            status: OllamaStatus::Starting { instant, timeout },
        }
        | OllamaStatusButtonVisualState::Pressed {
            status: OllamaStatus::Starting { instant, timeout },
        }) = button.visual_state
        else {
            continue;
        };
        sprite.color = Color::YELLOW
            * (1.0, 0.1).lerp(instant.elapsed().as_secs_f32() / timeout.as_secs_f32());
        for child in children.iter() {
            if let Ok(mut text) = button_text_query.get_mut(*child) {
                text.sections[0].value = format!(
                    "Ollama Server Control (Starting {}s ago)",
                    instant.elapsed().as_secs()
                );
            }
        }
    }
}

fn status_click_listener(
    mut click_events: EventReader<ClickEvent>,
    button_query: Query<&OllamaStatusButton>,
    mut status_events: EventWriter<OllamaStatusEvent>,
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
        if let Ok(button) = button_query.get(*target_id) {
            info!("Ollama Server Control clicked");
            // if the button visual status is alive, do nothing
            match button.visual_state {
                OllamaStatusButtonVisualState::Default {
                    status: OllamaStatus::Alive,
                }
                | OllamaStatusButtonVisualState::Hovered {
                    status: OllamaStatus::Alive,
                }
                | OllamaStatusButtonVisualState::Pressed {
                    status: OllamaStatus::Alive,
                } => {
                    warn!("Ollama Server Control is already alive");
                    continue;
                }
                _ => {}
            }
            let event = OllamaStatusEvent::Startup;
            debug!("Sending event {:?}", event);
            status_events.send(event);
        }
    }
}

fn terminal_click_listener(
    mut click_events: EventReader<ClickEvent>,
    button_query: Query<&OllamaTerminalButton>,
    mut terminal_events: EventWriter<OllamaTerminalEvent>,
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
        if button_query.get(*target_id).is_ok() {
            info!("Ollama terminal clicked");
            let event = OllamaTerminalEvent::Startup;
            debug!("Sending event {:?}", event);
            terminal_events.send(event);
        }
    }
}

fn handle_terminal_events(
    mut terminal_events: EventReader<OllamaTerminalEvent>,
) {
    let should_start = terminal_events.read().any(|event| {
        matches!(event, OllamaTerminalEvent::Startup)
    });
    if should_start {
        info!("Opening terminal");
        if let Err(e) = crate::ollama::start_terminal() {
            error!("Failed to start terminal: {:?}", e);
        }
    }
    terminal_events.clear();
}