use super::types::*;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ToolFrame;

#[allow(clippy::type_complexity)]
pub fn insert_hover_frame(
    mut reader: EventReader<ToolHoveredEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in reader.read() {
        match event {
            ToolHoveredEvent::HoverStart(entity) => {
                commands.entity(*entity).with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(200.0, 200.0)),
                                ..default()
                            },
                            texture: asset_server.load("textures/wood frame.png"),
                            ..default()
                        },
                        ToolFrame,
                    ));
                });
            }
            _ => {}
        }
    }
}
#[allow(clippy::type_complexity)]
pub fn remove_hover_frame(
    mut reader: EventReader<ToolHoveredEvent>,
    mut commands: Commands,
    tool_query: Query<&Children>,
    frame_query: Query<Entity, With<ToolFrame>>,
) {
    for event in reader.read() {
        match event {
            ToolHoveredEvent::HoverEnd(entity) => {
                if let Ok(children) = tool_query.get(*entity) {
                    for child in children.iter() {
                        if let Ok(frame) = frame_query.get(*child) {
                            commands.entity(frame).despawn();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
