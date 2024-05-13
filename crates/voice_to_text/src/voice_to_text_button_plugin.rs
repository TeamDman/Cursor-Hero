use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_cursor_types::prelude::*;
use cursor_hero_environment_types::prelude::*;
use cursor_hero_math::prelude::Lerp;
use cursor_hero_voice_to_text_types::prelude::*;

pub struct VoiceToTextButtonPlugin;

impl Plugin for VoiceToTextButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_new_host_environments);
        app.add_systems(Update, update_visuals);
        app.add_systems(Update, status_button_click);
        app.add_systems(Update, vscode_button_click);
        app.add_systems(Update, handle_vscode_events);
    }
}

fn populate_new_host_environments(
    mut commands: Commands,
    mut environment_events: EventReader<PopulateEnvironmentEvent>,
    environment_query: Query<(), With<HostEnvironment>>,
    asset_server: Res<AssetServer>,
) {
    for event in environment_events.read() {
        if !environment_query.contains(event.environment_id) {
            continue;
        }
        let environment_id = event.environment_id;
        info!("Adding button to new host environment {:?}", environment_id);
        commands.entity(environment_id).with_children(|parent| {
            parent
                .spawn((
                    VoiceToTextStatusButton::default(),
                    Name::new("Voice2Text Button"),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(200.0, 100.0)),
                            color: Color::PURPLE,
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            1920.0 / 2.0 + 600.0,
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
                            "Voice2Text Server Control".to_string(),
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
                    VoiceToTextVscodeButton::default(),
                    Name::new("Voice2Text VSCode Button"),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(200.0, 100.0)),
                            color: Color::rgb(0.0, 0.6, 0.8),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            1920.0 / 2.0 + 600.0,
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
                            "open in vscode".to_string(),
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
    mut events: EventReader<VoiceToTextStatusEvent>,
    mut button_query: Query<(&mut Sprite, &Children, &mut VoiceToTextStatusButton)>,
    mut button_text_query: Query<&mut Text>,
) {
    for event in events.read() {
        let VoiceToTextStatusEvent::Changed {
            new_status: status, ..
        } = event;
        debug!(
            "Updating VoiceToText Server Control visuals to {:?}",
            status
        );
        for button in button_query.iter_mut() {
            let (mut button_sprite, button_children, mut button) = button;
            button.visual_state = match button.visual_state {
                VoiceToTextStatusButtonVisualState::Default { .. } => {
                    VoiceToTextStatusButtonVisualState::Default {
                        status: status.clone(),
                    }
                }
                VoiceToTextStatusButtonVisualState::Hovered { .. } => {
                    VoiceToTextStatusButtonVisualState::Hovered {
                        status: status.clone(),
                    }
                }
                VoiceToTextStatusButtonVisualState::Pressed { .. } => {
                    VoiceToTextStatusButtonVisualState::Pressed {
                        status: status.clone(),
                    }
                }
            };
            match status {
                VoiceToTextStatus::Alive { .. } => {
                    button_sprite.color = Color::GREEN;
                }
                VoiceToTextStatus::AliveButWeDontKnowTheApiKey => {
                    button_sprite.color = Color::ORANGE_RED;
                }
                VoiceToTextStatus::Dead => {
                    button_sprite.color = Color::RED;
                }
                VoiceToTextStatus::Unknown | VoiceToTextStatus::UnknownWithCachedApiKey { .. } => {
                    button_sprite.color = Color::PURPLE;
                }
                VoiceToTextStatus::Starting {
                    instant, timeout, ..
                } => {
                    button_sprite.color = Color::YELLOW
                        * (1.0, 0.1).lerp(instant.elapsed().as_secs_f32() / timeout.as_secs_f32());
                }
            }
            for child in button_children.iter() {
                if let Ok(mut text) = button_text_query.get_mut(*child) {
                    match status {
                        VoiceToTextStatus::Alive { .. } => {
                            text.sections[0].value =
                                "VoiceToText Server Control (Alive)".to_string();
                        }
                        VoiceToTextStatus::AliveButWeDontKnowTheApiKey => {
                            text.sections[0].value =
                                "VoiceToText Server Control (Alive, but we don't know the API key)"
                                    .to_string();
                        }
                        VoiceToTextStatus::Dead => {
                            text.sections[0].value =
                                "VoiceToText Server Control (Dead)".to_string();
                        }
                        VoiceToTextStatus::Unknown => {
                            text.sections[0].value =
                                "VoiceToText Server Control (Unknown)".to_string();
                        }
                        VoiceToTextStatus::UnknownWithCachedApiKey { .. } => {
                            text.sections[0].value =
                                "VoiceToText Server Control (Unknown, api key present)".to_string();
                        }
                        VoiceToTextStatus::Starting { instant, .. } => {
                            text.sections[0].value = format!(
                                "VoiceToText Server Control (Starting {}s ago)",
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
        let (VoiceToTextStatusButtonVisualState::Default {
            status: VoiceToTextStatus::Starting {
                instant, timeout, ..
            },
        }
        | VoiceToTextStatusButtonVisualState::Hovered {
            status: VoiceToTextStatus::Starting {
                instant, timeout, ..
            },
        }
        | VoiceToTextStatusButtonVisualState::Pressed {
            status: VoiceToTextStatus::Starting {
                instant, timeout, ..
            },
        }) = button.visual_state
        else {
            continue;
        };
        sprite.color = Color::YELLOW
            * (1.0, 0.1).lerp(instant.elapsed().as_secs_f32() / timeout.as_secs_f32());
        for child in children.iter() {
            if let Ok(mut text) = button_text_query.get_mut(*child) {
                text.sections[0].value = format!(
                    "VoiceToText Server Control (Starting {}s ago)",
                    instant.elapsed().as_secs()
                );
            }
        }
    }
}

fn status_button_click(
    mut click_events: EventReader<ClickEvent>,
    button_query: Query<&VoiceToTextStatusButton>,
    mut command_events: EventWriter<VoiceToTextCommandEvent>,
) {
    for event in click_events.read() {
        let ClickEvent::Clicked { target_id, way, .. } = event else {
            continue;
        };
        if way != &Way::Left {
            continue;
        }
        if let Ok(button) = button_query.get(*target_id) {
            info!("VoiceToText Server Control clicked");
            // if the button visual status is alive, do nothing
            match button.visual_state {
                VoiceToTextStatusButtonVisualState::Default {
                    status: VoiceToTextStatus::Alive { .. },
                }
                | VoiceToTextStatusButtonVisualState::Hovered {
                    status: VoiceToTextStatus::Alive { .. },
                }
                | VoiceToTextStatusButtonVisualState::Pressed {
                    status: VoiceToTextStatus::Alive { .. },
                } => {
                    warn!("VoiceToText Server Control is already alive");
                    continue;
                }
                _ => {}
            }
            let event = VoiceToTextCommandEvent::Startup;
            debug!("Sending event {:?}", event);
            command_events.send(event);
        }
    }
}

fn vscode_button_click(
    mut click_events: EventReader<ClickEvent>,
    button_query: Query<&VoiceToTextVscodeButton>,
    mut vscode_events: EventWriter<VoiceToTextVscodeEvent>,
) {
    for event in click_events.read() {
        let ClickEvent::Clicked { target_id, way, .. } = event else {
            continue;
        };
        if way != &Way::Left {
            continue;
        }
        if button_query.get(*target_id).is_ok() {
            info!("VoiceToText vscode clicked");
            let event = VoiceToTextVscodeEvent::Startup;
            debug!("Sending event {:?}", event);
            vscode_events.send(event);
        }
    }
}

fn handle_vscode_events(mut vscode_events: EventReader<VoiceToTextVscodeEvent>) {
    let should_start = vscode_events
        .read()
        .any(|event| matches!(event, VoiceToTextVscodeEvent::Startup));
    if should_start {
        info!("Opening vscode");
        if let Err(e) = crate::voice_to_text::start_vscode() {
            error!("Failed to start vscode: {:?}", e);
        }
    }
    vscode_events.clear();
}
