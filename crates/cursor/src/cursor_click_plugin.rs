use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::HashSet;
use bevy_xpbd_2d::components::CollidingEntities;
use cursor_hero_cursor_types::prelude::*;

pub struct CursorClickPlugin;

impl Plugin for CursorClickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, press_detection);
        app.add_systems(Update, release_detection);
    }
}

#[allow(clippy::type_complexity)]
pub fn press_detection(
    mut commands: Commands,
    mut tool_click_events: EventReader<ToolClickEvent>,
    mut click_events: EventWriter<ClickEvent>,
    mut cursor_query: Query<
        (&GlobalTransform, &CollidingEntities, Option<&mut Pressing>),
        With<Cursor>,
    >,
    mut target_query: Query<(Entity, &Visibility, Option<&mut Pressed>), With<Clickable>>,
) {
    let mut cursor_target_ways: Vec<(Entity, Entity, Way, IVec2)> = vec![];
    let mut target_cursor_ways: Vec<(Entity, Entity, Way, IVec2)> = vec![];
    for tool_click_event in tool_click_events.read() {
        // only check pressed events
        let ToolClickEvent::Pressed { cursor_id, way } = tool_click_event else {
            continue;
        };

        // find the cursor for the event
        let Ok((cursor_global_transform, cursor_touching, cursor_pressing)) =
            cursor_query.get_mut(*cursor_id)
        else {
            warn!("Cursor {:?} not found", cursor_id);
            continue;
        };
        let cursor_global_position = cursor_global_transform.translation().truncate().as_ivec2();

        let mut pressed = vec![];

        // for each element the cursor is touching
        for touching_id in cursor_touching.iter() {
            // debug!("Cursor {:?} touching {:?}", cursor_id, touching_id);
            // ensure it is a clickable element
            let Ok((target_id, target_visible, target_pressed)) =
                target_query.get_mut(*touching_id)
            else {
                // debug!("Target {:?} not valid", touching_id);
                continue;
            };
            // ensure the element is visible
            if target_visible == Visibility::Hidden {
                continue;
            }

            // track in the element what is pressing it
            if target_pressed.is_none() {
                // nothing is pressing this element yet
                target_cursor_ways.push((*touching_id, *cursor_id, *way, cursor_global_position));
            } else if let Some(mut target_pressed) = target_pressed {
                // something is already pressing this element
                if target_pressed
                    .presses
                    .iter()
                    .any(|press| press.cursor_id == *cursor_id && press.way == *way)
                {
                    warn!("Cursor {:?} already pressing {:?}", cursor_id, target_id);
                } else {
                    target_pressed.presses.push(CursorPress {
                        cursor_id: *cursor_id,
                        way: *way,
                        start_position: cursor_global_position,
                    });
                }
            }

            // send pressed event
            debug!("Cursor {cursor_id:?} click pressed {way:?} on {target_id:?}");
            click_events.send(ClickEvent::Pressed {
                target_id,
                cursor_id: *cursor_id,
                way: *way,
                start_position: cursor_global_position,
            });

            pressed.push(target_id);
        }

        match cursor_pressing {
            Some(mut cursor_pressing) => {
                for target_id in pressed.into_iter() {
                    if cursor_pressing
                        .pressing
                        .iter()
                        .any(|p| p.target_id == target_id && p.way == *way)
                    {
                        warn!("Cursor {:?} already pressing {:?}", cursor_id, target_id);
                    } else {
                        cursor_pressing.pressing.push(TargetPress {
                            start_position: cursor_global_position,
                            target_id,
                            way: *way,
                        });
                    }
                }
            }
            None => {
                for target_id in pressed.into_iter() {
                    cursor_target_ways.push((*cursor_id, target_id, *way, cursor_global_position));
                }
            }
        }
    }

    // We have deferred the insertion of the Pressed and Pressing components
    // This is because doing it in the event loop causes clobbering when simultaneous events occur
    let cursor_target_ways = group_by_entity(cursor_target_ways);
    for (cursor_id, target_presses) in cursor_target_ways {
        commands.entity(cursor_id).insert(Pressing {
            pressing: target_presses
                .into_iter()
                .map(|(target_id, way, cursor_global_position)| TargetPress {
                    target_id,
                    way,
                    start_position: cursor_global_position,
                })
                .collect(),
        });
    }
    let target_cursor_ways = group_by_entity(target_cursor_ways);
    for (target_id, cursor_presses) in target_cursor_ways {
        commands.entity(target_id).insert(Pressed {
            presses: cursor_presses
                .into_iter()
                .map(|(cursor_id, way, cursor_global_position)| CursorPress {
                    cursor_id,
                    way,
                    start_position: cursor_global_position,
                })
                .collect(),
        });
    }
}

fn group_by_entity(
    ways: Vec<(Entity, Entity, Way, IVec2)>,
) -> HashMap<Entity, Vec<(Entity, Way, IVec2)>> {
    let mut groups: HashMap<Entity, Vec<(Entity, Way, IVec2)>> = HashMap::new();

    for (cursor, target, way, cursor_global_position) in ways {
        groups
            .entry(cursor)
            .or_insert_with(Vec::new)
            .push((target, way, cursor_global_position));
    }

    groups
}

#[allow(clippy::type_complexity)]
fn release_detection(
    mut commands: Commands,
    mut tool_click_events: EventReader<ToolClickEvent>,
    mut click_events: EventWriter<ClickEvent>,
    mut cursor_query: Query<
        (&GlobalTransform, &CollidingEntities, Option<&mut Pressing>),
        With<Cursor>,
    >,
    mut target_query: Query<(Entity, &Visibility, Option<&mut Pressed>), With<Clickable>>,
) {
    for tool_click_event in tool_click_events.read() {
        // only check released events
        let ToolClickEvent::Released { cursor_id, way } = tool_click_event else {
            continue;
        };

        // find the cursor for the event
        let Ok((cursor_global_transform, cursor_touching, cursor_pressing)) =
            cursor_query.get_mut(*cursor_id)
        else {
            warn!("Cursor {:?} not found", cursor_id);
            continue;
        };
        let cursor_global_position = cursor_global_transform.translation().truncate().as_ivec2();

        // each element the cursor has tracked as pressing now needs to be released
        // if the cursor is still touching that element, also send a click event

        // for each element the cursor is touching
        let mut clicked = vec![];
        for touching_id in cursor_touching.iter() {
            // ensure it is a clickable element
            let Ok((target_id, target_visible, target_pressed)) =
                target_query.get_mut(*touching_id)
            else {
                continue;
            };
            // ensure the element is visible
            if target_visible == Visibility::Hidden {
                continue;
            }

            // update the tracker in the target
            if let Some(mut pressed) = target_pressed {
                if let Some(press_index) = pressed
                    .presses
                    .iter()
                    .position(|press| press.cursor_id == *cursor_id && press.way == *way)
                {
                    if pressed.presses.len() == 1 {
                        // this is the last press, remove the tracker
                        commands.entity(target_id).remove::<Pressed>();
                    } else {
                        // remove the press from the tracker
                        pressed.presses.remove(press_index);
                    }
                } else {
                    warn!("Cursor {:?} not pressing {:?}", cursor_id, target_id);
                }
            } else {
                warn!(
                    "Target {:?} didn't know it was pressed by cursor {:?}. Did you press elsewhere and release here?",
                    target_id, cursor_id
                );
            }

            clicked.push(target_id);
        }

        let Some(mut pressing) = cursor_pressing else {
            warn!("Cursor {:?} wasn't pressing anything", cursor_id);
            continue;
        };

        // send release events
        let mut remove = HashSet::new();
        pressing
            .pressing
            .iter()
            .filter(|press| press.way == *way)
            .for_each(|press: &TargetPress| {
                debug!(
                    "Cursor {cursor_id:?} click released {way:?} on {:?}",
                    press.target_id
                );
                click_events.send(ClickEvent::Released {
                    start_position: press.start_position,
                    end_position: cursor_global_position,
                    target_id: press.target_id,
                    cursor_id: *cursor_id,
                    way: *way,
                });
                remove.insert(*press);
            });
        pressing.pressing.retain(|press| !remove.contains(press));

        if pressing.pressing.is_empty() {
            commands.entity(*cursor_id).remove::<Pressing>();
        }
        // only perform full clicked event for released presses
        for target_id in clicked.iter() {
            let press = remove
                .iter()
                .find(|press| press.target_id == *target_id && press.way == *way);
            let Some(press) = press else {
                warn!(
                    "Cursor {:?} didn't know it was clicking {:?}",
                    cursor_id, target_id
                );
                continue;
            };
            debug!("Cursor {cursor_id:?} clicked {way:?} on {target_id:?}");
            click_events.send(ClickEvent::Clicked {
                target_id: *target_id,
                cursor_id: *cursor_id,
                start_position: press.start_position,
                end_position: cursor_global_position,
                way: *way,
            });
        }
    }
}
