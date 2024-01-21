use super::types::*;
use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use bevy_xpbd_2d::components::Sensor;
use cursor_hero_bevy::NameOrEntityDisplay;
use leafwing_input_manager::action_state::ActionState;

#[allow(clippy::type_complexity)]
pub fn tool_help_lifecycle(
    mut commands: Commands,
    toolbelt_query: Query<
        (&ActionState<ToolbeltAction>, &Children),
        (Without<Tool>, With<Toolbelt>),
    >,
    tool_query: Query<(Entity, Option<&Children>, Option<&Name>), With<Tool>>,
    tool_help_triggger_query: Query<Entity, With<ToolHelpTrigger>>,
    asset_server: Res<AssetServer>,
) {
    for (toolbelt_actions, toolbelt_kids) in toolbelt_query.iter() {
        if toolbelt_actions.just_pressed(ToolbeltAction::Show) {
            for child_id in toolbelt_kids.iter() {
                if let Ok((tool_id, _, tool_name)) = tool_query.get(*child_id) {
                    commands.entity(tool_id).with_children(|parent| {
                        parent.spawn((
                            Name::new(format!(
                                "Help Trigger for {}",
                                tool_name.name_or_entity(tool_id)
                            )),
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(50.0, 50.0)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                texture: asset_server
                                    .load("textures/toolbelt_wheel/help_trigger.png"),
                                ..default()
                            },
                            ToolHelpTrigger,
                            Sensor,
                            RigidBody::Kinematic,
                            Collider::cuboid(50.0, 50.0),
                        ));
                    });
                }
            }
        } else if toolbelt_actions.just_released(ToolbeltAction::Show) {
            for child_id in toolbelt_kids.iter() {
                if let Ok((tool_id, Some(tool_children), _)) = tool_query.get(*child_id) {
                    for child_id in tool_children.iter() {
                        if let Ok(tool_help_trigger_id) = tool_help_triggger_query.get(*child_id) {
                            commands
                                .entity(tool_id)
                                .remove_children(&[tool_help_trigger_id]);
                            commands.entity(tool_help_trigger_id).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}