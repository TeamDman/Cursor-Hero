use bevy::prelude::*;
use cursor_hero_cursor_types::prelude::*;

#[derive(Component, Debug)]
pub struct ToolFrame;

#[allow(clippy::type_complexity)]
pub fn insert_hover_frame(
    mut reader: EventReader<HoverEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hovered_query: Query<&Sprite>,
) {
    for event in reader.read() {
        if let HoverEvent::Start {
            target_id,
            pointer_id: _,
        } = event
        {
            // Ensure the entity hasn't despawned since the event was sent
            if commands.get_entity(*target_id).is_none() {
                warn!("Hovered entity {:?} has despawned", target_id);
                continue;
            }
            let mut size = Vec2::new(200.0, 200.0);
            if let Ok(hovered_sprite) = hovered_query.get(*target_id)
                && let Some(hovered_size) = hovered_sprite.custom_size
            {
                size = hovered_size * 2.0;
            }

            commands.entity(*target_id).with_children(|hovered| {
                hovered.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(size),
                            ..default()
                        },
                        texture: asset_server.load("textures/wood frame.png"),
                        ..default()
                    },
                    ToolFrame,
                ));
            });
        }
    }
}
#[allow(clippy::type_complexity)]
pub fn remove_hover_frame(
    mut reader: EventReader<HoverEvent>,
    mut commands: Commands,
    tool_query: Query<&Children>,
    frame_query: Query<Entity, With<ToolFrame>>,
) {
    for event in reader.read() {
        if let HoverEvent::End {
            target_id,
            pointer_id: _,
        } = event
        {
            if let Ok(tool_children) = tool_query.get(*target_id) {
                for tool_child in tool_children.iter() {
                    if let Ok(frame_id) = frame_query.get(*tool_child) {
                        commands.entity(*target_id).remove_children(&[frame_id]);
                        commands.entity(frame_id).despawn();
                    }
                }
            }
        }
    }
}
