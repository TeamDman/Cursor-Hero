use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_environment_types::prelude::*;
use cursor_hero_glados_tts_types::prelude::*;
use cursor_hero_math::Lerp;
use cursor_hero_pointer_types::prelude::*;
pub struct GladosTtsButtonPlugin;

impl Plugin for GladosTtsButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_new_host_environments);
        app.add_systems(Update, update_visuals);
        app.add_systems(Update, click_listener);
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
        info!("Adding button to new host environment {:?}", environment_id);
        commands.entity(*environment_id).with_children(|parent| {
            parent
                .spawn((
                    GladosTtsStatusButton::default(),
                    Name::new("GLaDOS TTS Button"),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(200.0, 100.0)),
                            color: Color::PURPLE,
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            1920.0 / 2.0 - 600.0,
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
                            "GLaDOS TTS Server Control".to_string(),
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
        });
    }
}

fn update_visuals(
    mut events: EventReader<GladosTtsStatusEvent>,
    mut button_query: Query<(&mut Sprite, &Children, &mut GladosTtsStatusButton)>,
    mut button_text_query: Query<&mut Text>,
) {
    for event in events.read() {
        let GladosTtsStatusEvent::Changed { new_value: status } = event else {
            continue;
        };
        debug!("Updating GladosTts Server Control visuals to {:?}", status);
        for button in button_query.iter_mut() {
            let (mut button_sprite, button_children, mut button) = button;
            button.visual_state = match button.visual_state {
                GladosTtsStatusButtonVisualState::Default { .. } => {
                    GladosTtsStatusButtonVisualState::Default { status: *status }
                }
                GladosTtsStatusButtonVisualState::Hovered { .. } => {
                    GladosTtsStatusButtonVisualState::Hovered { status: *status }
                }
                GladosTtsStatusButtonVisualState::Pressed { .. } => {
                    GladosTtsStatusButtonVisualState::Pressed { status: *status }
                }
            };
            match status {
                GladosTtsStatus::Alive => {
                    button_sprite.color = Color::GREEN;
                }
                GladosTtsStatus::Dead => {
                    button_sprite.color = Color::RED;
                }
                GladosTtsStatus::Unknown => {
                    button_sprite.color = Color::PURPLE;
                }
                GladosTtsStatus::Starting { instant, timeout } => {
                    button_sprite.color = Color::YELLOW
                        * (1.0, 0.1)
                            .lerp(instant.elapsed().as_secs_f32() / timeout.as_secs_f32());
                }
            }
            for child in button_children.iter() {
                if let Ok(mut text) = button_text_query.get_mut(*child) {
                    match status {
                        GladosTtsStatus::Alive => {
                            text.sections[0].value =
                                "GladosTts Server Control (Alive)".to_string();
                        }
                        GladosTtsStatus::Dead => {
                            text.sections[0].value =
                                "GladosTts Server Control (Dead)".to_string();
                        }
                        GladosTtsStatus::Unknown => {
                            text.sections[0].value =
                                "GladosTts Server Control (Unknown)".to_string();
                        }
                        GladosTtsStatus::Starting { instant, .. } => {
                            text.sections[0].value = format!(
                                "GladosTts Server Control (Starting {}s ago)",
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
        let (GladosTtsStatusButtonVisualState::Default {
            status: GladosTtsStatus::Starting { instant, timeout },
        }
        | GladosTtsStatusButtonVisualState::Hovered {
            status: GladosTtsStatus::Starting { instant, timeout },
        }
        | GladosTtsStatusButtonVisualState::Pressed {
            status: GladosTtsStatus::Starting { instant, timeout },
        }) = button.visual_state
        else {
            continue;
        };
        sprite.color = Color::YELLOW
            * (1.0, 0.1).lerp(instant.elapsed().as_secs_f32() / timeout.as_secs_f32());
        for child in children.iter() {
            if let Ok(mut text) = button_text_query.get_mut(*child) {
                text.sections[0].value = format!(
                    "GladosTts Server Control (Starting {}s ago)",
                    instant.elapsed().as_secs()
                );
            }
        }
    }
}

fn click_listener(
    mut click_events: EventReader<ClickEvent>,
    button_query: Query<&GladosTtsStatusButton>,
    mut status_events: EventWriter<GladosTtsStatusEvent>,
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
            info!("GladosTts Server Control clicked");
            // if the button visual status is alive, do nothing
            match button.visual_state {
                GladosTtsStatusButtonVisualState::Default {
                    status: GladosTtsStatus::Alive,
                }
                | GladosTtsStatusButtonVisualState::Hovered {
                    status: GladosTtsStatus::Alive,
                }
                | GladosTtsStatusButtonVisualState::Pressed {
                    status: GladosTtsStatus::Alive,
                } => {
                    warn!("GladosTts Server Control is already alive");
                    continue;
                }
                _ => {}
            }
            let event = GladosTtsStatusEvent::Startup;
            debug!("Sending event {:?}", event);
            status_events.send(event);
        }
    }
}
