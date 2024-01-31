use bevy::prelude::*;
use bevy_xpbd_2d::components::CollidingEntities;
use cursor_hero_pointer_types::prelude::*;

pub struct PointerHoverPlugin;

impl Plugin for PointerHoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hover_detection);
    }
}


#[allow(clippy::type_complexity)]
pub fn hover_detection(
    mut commands: Commands,
    mut pointer_query: Query<(Entity, &CollidingEntities, Option<&mut Hovering>), With<Pointer>>,
    target_query: Query<(Entity, &Visibility, Option<&Hovered>), With<Hoverable>>,
    mut events: EventWriter<HoverEvent>,
) {
    for (pointer_id, pointer_touching, pointer_hovering) in pointer_query.iter_mut() {
        // find out what the pointer is touching
        let mut still_touching = vec![];
        for touching_id in pointer_touching.iter() {
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
                    pointer_id,
                });
            }
            still_touching.push(target_id);
        }
        // update the pointer tracker
        match pointer_hovering {
            Some(mut pointer_hovering) => {
                for entry in pointer_hovering.hovering.iter() {
                    if !still_touching.contains(entry) {
                        if let Some(mut target_commands) = commands.get_entity(*entry) {
                            target_commands.remove::<Hovered>();
                            debug!("HoverEnd: {:?}", entry);
                            events.send(HoverEvent::End {
                                target_id: *entry,
                                pointer_id,
                            });
                        }
                    }
                }
                if still_touching.is_empty() {
                    commands.entity(pointer_id).remove::<Hovering>();
                } else {
                    pointer_hovering.hovering = still_touching;
                }
            }
            None => {
                if !still_touching.is_empty() {
                    commands.entity(pointer_id).insert(Hovering {
                        hovering: still_touching,
                    });
                }
            }
        }
    }
}
