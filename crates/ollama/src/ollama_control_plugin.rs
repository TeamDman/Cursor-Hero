use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_environment_types::prelude::*;
use cursor_hero_math::Lerp;
use cursor_hero_ollama_types::prelude::*;
use cursor_hero_pointer_types::prelude::*;
pub struct OllamaControlPlugin;

impl Plugin for OllamaControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_control_to_new_host_environments);
        app.add_systems(Update, update_control_visuals);
        app.add_systems(Update, click_listener);
    }
}

fn add_control_to_new_host_environments(
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
                    parent.spawn((
                        Text2dBundle {
                            text: Text::from_section(
                                "Ollama Server Control".to_string(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            transform: Transform::from_xyz(0.0, 70.0, 1.0),
                            ..default()
                        },
                        Name::new("Ollama Server Control Text"),
                    ));
                });
        });
    }
}

fn update_control_visuals(
    mut events: EventReader<OllamaStatusEvent>,
    mut button_query: Query<(&mut Sprite, &Children, &mut OllamaStatusButton)>,
    mut button_text_query: Query<&mut Text>,
) {
    for event in events.read() {
        match event {
            OllamaStatusEvent::Changed { new_value: status } => {
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
                                * (1.0, 0.1)
                                    .lerp(instant.elapsed().as_secs_f32() / timeout.as_secs_f32());
                        }
                    }
                    for child in button_children.iter() {
                        if let Ok(mut text) = button_text_query.get_mut(*child) {
                            match status {
                                OllamaStatus::Alive => {
                                    text.sections[0].value =
                                        "Ollama Server Control (Alive)".to_string();
                                }
                                OllamaStatus::Dead => {
                                    text.sections[0].value =
                                        "Ollama Server Control (Dead)".to_string();
                                }
                                OllamaStatus::Unknown => {
                                    text.sections[0].value =
                                        "Ollama Server Control (Unknown)".to_string();
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
            _ => {}
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

fn click_listener(
    mut click_events: EventReader<ClickEvent>,
    button_query: Query<Entity, With<OllamaStatusButton>>,
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
        if let Ok(_) = button_query.get(*target_id) {
            info!("Ollama Server Control clicked");
            let event = OllamaStatusEvent::Startup;
            debug!("Sending event {:?}", event);
            status_events.send(event);
        }
    }
}
