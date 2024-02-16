use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_chat_types::prelude::*;
pub struct ChatBubblePlugin;

impl Plugin for ChatBubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_chat_input_events);
        app.add_systems(Update, handle_chat_events);
        app.add_systems(Update, chat_bubble_lifetime);
    }
}
fn handle_chat_input_events(
    mut commands: Commands,
    tool_query: Query<&mut ChatTool>,
    character_query: Query<&Children, With<Character>>,
    chat_input_query: Query<&Children, With<ChatInput>>,
    mut text_query: Query<&mut Text>,
    mut chat_input_events: EventReader<ChatInputEvent>,
) {
    for event in chat_input_events.read() {
        match event {
            ChatInputEvent::Focus {
                character_id,
                tool_id,
                ..
            } => {
                if let Ok(character_children) = character_query.get(*character_id) {
                    for child in character_children.iter() {
                        if chat_input_query.get(*child).is_ok() {
                            warn!("Chat input entity already exists?");
                            continue;
                        }
                    }
                }
                let starting_text = match tool_query.get(*tool_id) {
                    Ok(tool) => tool.buffer.clone(),
                    Err(_) => {
                        warn!(
                            "Chat tool {:?} not found? Skipping chat bubble creation.",
                            tool_id
                        );
                        continue;
                    }
                };
                debug!(
                    "Creating chat input entity for character {:?}",
                    character_id
                );
                commands.entity(*character_id).with_children(|parent| {
                    let size = Vec2::new(300.0, 100.0);
                    let resolution = 3.0;
                    let padding = Vec2::new(10.0, 10.0);
                    parent
                        .spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::ALICE_BLUE,
                                    custom_size: Some(size),
                                    ..default()
                                },
                                transform: Transform::from_translation(Vec3::new(0.0, 100.0, -1.0)),
                                ..default()
                            },
                            ChatInput,
                            Name::new("Chat Input Bubble"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text2dBundle {
                                text: Text::from_section(
                                    starting_text,
                                    TextStyle {
                                        font_size: 20.0 * resolution,
                                        color: Color::MIDNIGHT_BLUE,
                                        ..default()
                                    },
                                ),
                                text_2d_bounds: Text2dBounds {
                                    size: size * resolution - padding,
                                },
                                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                                    .with_scale(Vec3::new(1.0 / resolution, 1.0 / resolution, 1.0)),
                                ..default()
                            },));
                        });
                });
            }
            ChatInputEvent::Unfocus { character_id, .. } => {
                debug!(
                    "Removing chat input entity for character {:?}",
                    character_id
                );
                if let Ok(character_children) = character_query.get(*character_id) {
                    for child in character_children.iter() {
                        if chat_input_query.get(*child).is_ok() {
                            commands.entity(*child).despawn_recursive();
                            commands.entity(*character_id).remove_children(&[*child]);
                        }
                    }
                }
            }
            ChatInputEvent::TextChanged {
                character_id,
                tool_id,
                ..
            } => {
                let new_text = match tool_query.get(*tool_id) {
                    Ok(tool) => tool.buffer.clone(),
                    Err(_) => {
                        warn!(
                            "Chat tool {:?} not found? Skipping chat bubble update.",
                            tool_id
                        );
                        continue;
                    }
                };
                debug!(
                    "Updating chat input entity for character {:?}",
                    character_id
                );
                if let Ok(character_children) = character_query.get(*character_id) {
                    for child in character_children.iter() {
                        if let Ok(chat_input) = chat_input_query.get(*child) {
                            let chat_input_children = chat_input;
                            for child in chat_input_children.iter() {
                                if let Ok(mut text) = text_query.get_mut(*child) {
                                    text.sections[0].value = new_text.clone();
                                }
                            }
                        }
                    }
                } else {
                    warn!(
                        "Character {:?} not found? Skipping chat bubble update.",
                        character_id
                    );
                }
            }
        }
    }
}

fn handle_chat_events(
    mut events: EventReader<ChatEvent>,
    mut commands: Commands,
    character_query: Query<&GlobalTransform, With<Character>>,
) {
    for event in events.read() {
        match event {
            ChatEvent::Chat {
                character_id,
                message,
            } => {
                if let Ok(character) = character_query.get(*character_id) {
                    let character_transform = character;
                    info!(
                        "Creating chat bubble for character {:?} at position {:?}",
                        character_id,
                        character_transform.translation()
                    );
                    let size = Vec2::new(300.0, 100.0);
                    let resolution = 3.0;
                    let padding = Vec2::new(10.0, 10.0);
                    let mut transform = character_transform.compute_transform();
                    transform.translation -= Vec3::new(0.0, 100.0, 10.0);
                    commands
                        .spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BLACK,
                                    custom_size: Some(size),
                                    ..default()
                                },
                                transform,
                                ..default()
                            },
                            ChatBubble {
                                lifetime: Timer::from_seconds(25.0, TimerMode::Once),
                            },
                            RigidBody::Dynamic,
                            LinearVelocity(Vec2::new(0.0, -30.0)),
                            Collider::cuboid(size.x, size.y),
                            Name::new("Chat Bubble"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text2dBundle {
                                text: Text::from_section(
                                    message.clone(),
                                    TextStyle {
                                        font_size: 20.0 * resolution,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                                text_2d_bounds: Text2dBounds {
                                    size: size * resolution - padding,
                                },
                                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                                    .with_scale(Vec3::new(1.0 / resolution, 1.0 / resolution, 1.0)),
                                ..default()
                            },));
                        });
                } else {
                    warn!(
                        "Character {:?} not found? Skipping chat bubble creation.",
                        character_id
                    );
                }
            }
        }
    }
}

fn chat_bubble_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ChatBubble)>,
) {
    for (entity, mut chat_bubble) in query.iter_mut() {
        chat_bubble.lifetime.tick(time.delta());
        if chat_bubble.lifetime.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
