use super::types::*;
use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use cursor_hero_xelu_prompts::texture_path_for_input;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

pub fn tool_help_activation(
    mut commands: Commands,
    toolbelt_query: Query<(&ActionState<ToolbeltAction>, &Children, &GlobalTransform)>,
    tool_query: Query<(&Tool, &Children)>,
    hovered_query: Query<&GlobalTransform, (With<Hovered>, With<ToolHelpTrigger>)>,
    asset_server: Res<AssetServer>,
) {
    for (toolbelt_actions, toolbelt_children, toolbelt_transform) in toolbelt_query.iter() {
        if toolbelt_actions.just_released(ToolbeltAction::Show) {
            // check all the toolbelt children
            for tool_id in toolbelt_children {
                // if the child is a tool
                if let Ok((tool, tool_children)) = tool_query.get(*tool_id) {
                    // and the tool has children
                    for tool_child_id in tool_children.iter() {
                        // and the hovered child is a tool help trigger
                        if let Ok(hovered_transform) = hovered_query.get(*tool_child_id) {
                            let toolbelt_position = toolbelt_transform.translation();
                            let hovered_position = hovered_transform.translation();
                            let look = hovered_position - toolbelt_position;
                            let position = hovered_position + look;
                            spawn_help_for_tool(&mut commands, position, tool, &asset_server);
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_help_for_tool(
    commands: &mut Commands,
    position: Vec3,
    tool: &Tool,
    asset_server: &Res<AssetServer>,
) {
    info!("Spawning help for tool: {:?}", tool.name);
    let mut parent_commands = commands.spawn((
        Name::new(format!("Help for {}", tool.name)),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                color: Color::rgba(0.5, 0.5, 1.0, 0.8),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        ToolHelp {
            timer: Timer::from_seconds(25.0, TimerMode::Once),
        },
        RigidBody::Dynamic,
        Collider::cuboid(100.0, 100.0),
    ));
    parent_commands.with_children(|parent| {
        // image
        parent.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture: tool.texture.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        });
        // name
        parent.spawn(Text2dBundle {
            text: Text::from_section(
                tool.name.clone(),
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0.0, 60.0, 1.0),
            ..default()
        });
        // description
        parent.spawn(Text2dBundle {
            text: Text::from_section(
                tool.description.clone(),
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0.0, -60.0, 1.0),
            ..default()
        });

        // actions
        let action_start_y = -100.0; // Starting y position for actions
        let action_spacing_y = 40.0; // Space between each action
        let action_name_x = -150.0;
        let key_size = 50.0; // Size of each key
        let key_spacing_x = key_size + 15.0; // Space between each key
        for (i, (action_name, action_inputs)) in tool.actions.iter().enumerate() {
            let action_y = action_start_y - (i as f32 * action_spacing_y);

            // Action name text
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    format!("{:?}", action_name),
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(action_name_x, action_y, 0.2), // Place text to the left
                ..default()
            });

            // Keys for action
            let mut key_x = 25.0; // Starting x position for keys
            for action in action_inputs.iter() {
                let key_position = Vec3::new(key_x, action_y, 0.2); // Calculate the position for the key
                key_x += key_spacing_x; // Move the x position for the next key

                match action {
                    UserInput::Single(kind) => match texture_path_for_input(kind) {
                        Some(path) => {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(key_size, key_size)),
                                    ..default()
                                },
                                texture: asset_server.load(path),
                                transform: Transform::from_translation(key_position),
                                ..default()
                            });
                        }
                        None => {
                            warn!("No texture for input: {:?}", kind);
                        }
                    },
                    _ => {
                        warn!("Only single inputs are supported for tool help");
                    }
                }
            }
        }
    });
}
