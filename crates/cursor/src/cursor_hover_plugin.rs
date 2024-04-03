use bevy::prelude::*;
use bevy_xpbd_2d::components::CollidingEntities;
use cursor_hero_cursor_types::prelude::*;

pub struct CursorHoverPlugin;

impl Plugin for CursorHoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hover_detection);
    }
}

#[allow(clippy::type_complexity)]
pub fn hover_detection(
    mut commands: Commands,
    mut cursor_query: Query<(Entity, &CollidingEntities, Option<&mut Hovering>), With<Cursor>>,
    target_query: Query<(Entity, &Visibility, Option<&Hovered>), With<Hoverable>>,
    mut events: EventWriter<HoverEvent>,
) {
    for (cursor_id, cursor_touching, cursor_hovering) in cursor_query.iter_mut() {
        // find out what the cursor is touching
        let mut still_touching = vec![];
        for touching_id in cursor_touching.iter() {
            let Ok((target_id, target_visible, target_hovered)) = target_query.get(*touching_id)
            else {
                continue;
            };
            if target_visible == Visibility::Hidden {
                continue;
            }
            if target_hovered.is_none() {
                commands.entity(target_id).insert(Hovered);
                debug!("HoverStart: {:?}", target_id);
                events.send(HoverEvent::Start {
                    target_id,
                    cursor_id,
                });
            }
            still_touching.push(target_id);
        }
        // update the cursor tracker
        match cursor_hovering {
            Some(mut cursor_hovering) => {
                for entry in cursor_hovering.hovering.iter() {
                    if !still_touching.contains(entry) {
                        if let Some(mut target_commands) = commands.get_entity(*entry) {
                            target_commands.remove::<Hovered>();
                            debug!("HoverEnd: {:?}", entry);
                            events.send(HoverEvent::End {
                                target_id: *entry,
                                cursor_id,
                            });
                        }
                    }
                }
                if still_touching.is_empty() {
                    commands.entity(cursor_id).remove::<Hovering>();
                } else {
                    cursor_hovering.hovering = still_touching;
                }
            }
            None => {
                if !still_touching.is_empty() {
                    commands.entity(cursor_id).insert(Hovering {
                        hovering: still_touching,
                    });
                }
            }
        }
    }
}
