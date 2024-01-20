use super::types::*;
use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use bevy_xpbd_2d::components::Sensor;
use leafwing_input_manager::prelude::*;

pub fn tool_help_activation(
    mut commands: Commands,
    toolbelt_query: Query<(&ActionState<ToolbeltAction>, &Children, &GlobalTransform)>,
    tool_query: Query<(&Tool, &Children)>,
    hovered_query: Query<&GlobalTransform, (With<Hovered>, With<ToolHelpTrigger>)>,
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
                            spawn_help_for_tool(&mut commands, position, tool);
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_help_for_tool(commands: &mut Commands, position: Vec3, tool: &Tool) {
    info!("Spawning help for tool: {:?}", tool.name);
    let mut parent_commands = commands.spawn((
        Name::new(format!("Help for {}", tool.name)),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                // color: Color::rgba(0.5, 0.5, 1.0, 0.5),
                ..default()
            },
            texture: tool.texture.clone(),
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
            transform: Transform::from_xyz(0.0, 60.0, 0.0),
            ..default()
        });
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
            transform: Transform::from_xyz(0.0, -60.0, 0.0),
            ..default()
        });
    });
}
