use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_xpbd_2d::components::CollidingEntities;

use crate::pointer_plugin::Pointer;

pub struct PointerClickPlugin;

impl Plugin for PointerClickPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Clickable>();
        app.register_type::<Pressed>();
        app.register_type::<Pressing>();
        app.add_event::<ClickEvent>();
        app.add_event::<ToolClickEvent>();
        app.add_systems(Update, press_detection);
        app.add_systems(Update, release_detection);
    }
}

#[derive(Component, Reflect, Debug)]
pub struct Clickable;

#[derive(Reflect, Debug)]
struct PointerPress {
    pointer_id: Entity,
    way: Way,
}
#[derive(Component, Reflect, Debug)]
pub struct Pressed {
    presses: Vec<PointerPress>,
}

#[derive(Reflect, Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct TargetPress {
    target_id: Entity,
    way: Way,
}
#[derive(Component, Reflect, Debug)]
pub struct Pressing {
    pressing: Vec<TargetPress>,
}

#[derive(Reflect, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Way {
    Left,
    Right,
    Middle,
}

#[derive(Event, Debug, Reflect)]
pub enum ClickEvent {
    Pressed {
        target_id: Entity,
        pointer_id: Entity,
        way: Way,
    },
    Released {
        target_id: Entity,
        pointer_id: Entity,
        way: Way,
    },
    Clicked {
        target_id: Entity,
        pointer_id: Entity,
        way: Way,
    },
}
#[derive(Event, Debug, Reflect)]
pub enum ToolClickEvent {
    Pressed { pointer_id: Entity, way: Way },
    Released { pointer_id: Entity, way: Way },
}

#[allow(clippy::type_complexity)]
pub fn press_detection(
    mut commands: Commands,
    mut tool_click_events: EventReader<ToolClickEvent>,
    mut click_events: EventWriter<ClickEvent>,
    mut pointer_query: Query<(&CollidingEntities, Option<&mut Pressing>), With<Pointer>>,
    mut target_query: Query<(Entity, &Visibility, Option<&mut Pressed>), With<Clickable>>,
) {
    for tool_click_event in tool_click_events.read() {
        // only check pressed events
        let ToolClickEvent::Pressed { pointer_id, way } = tool_click_event else {
            continue;
        };

        // find the pointer for the event
        let Ok((pointer_touching, pointer_pressing)) = pointer_query.get_mut(*pointer_id) else {
            warn!("Pointer {:?} not found", pointer_id);
            continue;
        };

        let mut pressed = vec![];

        // for each element the pointer is touching
        for touching_id in pointer_touching.iter() {
            // debug!("Pointer {:?} touching {:?}", pointer_id, touching_id);
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
                // note: two presses on the same frame will cause a clobber to occur here
                commands.entity(target_id).insert(Pressed {
                    presses: vec![PointerPress {
                        pointer_id: *pointer_id,
                        way: *way,
                    }],
                });
            } else if let Some(mut target_pressed) = target_pressed {
                // something is already pressing this element
                if target_pressed
                    .presses
                    .iter()
                    .any(|press| press.pointer_id == *pointer_id && press.way == *way)
                {
                    warn!("Pointer {:?} already pressing {:?}", pointer_id, target_id);
                } else {
                    target_pressed.presses.push(PointerPress {
                        pointer_id: *pointer_id,
                        way: *way,
                    });
                }
            }

            // send pressed event
            click_events.send(ClickEvent::Pressed {
                target_id,
                pointer_id: *pointer_id,
                way: *way,
            });

            pressed.push(target_id);
        }

        match pointer_pressing {
            Some(mut pointer_pressing) => {
                for target_id in pressed.into_iter() {
                    if pointer_pressing
                        .pressing
                        .iter()
                        .any(|p| p.target_id == target_id && p.way == *way)
                    {
                        warn!("Pointer {:?} already pressing {:?}", pointer_id, target_id);
                    } else {
                        pointer_pressing.pressing.push(TargetPress {
                            target_id,
                            way: *way,
                        });
                    }
                }
            }
            None => {
                for target_id in pressed.into_iter() {
                    // note: two presses on the same frame will cause a clobber to occur here
                    // TODO: fix click clobbering
                    commands.entity(*pointer_id).insert(Pressing {
                        pressing: vec![TargetPress {
                            target_id,
                            way: *way,
                        }],
                    });
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn release_detection(
    mut commands: Commands,
    mut tool_click_events: EventReader<ToolClickEvent>,
    mut click_events: EventWriter<ClickEvent>,
    mut pointer_query: Query<(&CollidingEntities, Option<&mut Pressing>), With<Pointer>>,
    mut target_query: Query<(Entity, &Visibility, Option<&mut Pressed>), With<Clickable>>,
) {
    for tool_click_event in tool_click_events.read() {
        // only check released events
        let ToolClickEvent::Released { pointer_id, way } = tool_click_event else {
            continue;
        };

        // find the pointer for the event
        let Ok((pointer_touching, pointer_pressing)) = pointer_query.get_mut(*pointer_id) else {
            warn!("Pointer {:?} not found", pointer_id);
            continue;
        };

        // each element the pointer has tracked as pressing now needs to be released
        // if the pointer is still touching that element, also send a click event

        // for each element the pointer is touching
        let mut clicked: Vec<Entity> = vec![];
        for touching_id in pointer_touching.iter() {
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
                    .position(|press| press.pointer_id == *pointer_id && press.way == *way)
                {
                    if pressed.presses.len() == 1 {
                        // this is the last press, remove the tracker
                        commands.entity(target_id).remove::<Pressed>();
                    } else {
                        // remove the press from the tracker
                        pressed.presses.remove(press_index);
                    }
                } else {
                    warn!("Pointer {:?} not pressing {:?}", pointer_id, target_id);
                }
            } else {
                warn!(
                    "Target {:?} didn't know it was pressed by pointer {:?}. Did you press elsewhere and release here?",
                    target_id, pointer_id
                );
            }

            clicked.push(target_id);
        }

        match pointer_pressing {
            Some(mut pressing) => {
                // send release events
                let mut remove = HashSet::new();
                pressing
                    .pressing
                    .iter()
                    .filter(|press| press.way == *way)
                    .for_each(|press: &TargetPress| {
                        click_events.send(ClickEvent::Released {
                            target_id: press.target_id,
                            pointer_id: *pointer_id,
                            way: *way,
                        });
                        remove.insert(*press);
                    });
                pressing.pressing.retain(|press| !remove.contains(press));
                if pressing.pressing.is_empty() {
                    commands.entity(*pointer_id).remove::<Pressing>();
                }
                // ensure all clicked are present in remove
                for target_id in clicked.iter() {
                    if !remove.contains(&TargetPress {
                        target_id: *target_id,
                        way: *way,
                    }) {
                        warn!(
                            "Pointer {:?} didn't know it was clicking {:?}",
                            pointer_id, target_id
                        );
                    } else {
                        click_events.send(ClickEvent::Clicked {
                            target_id: *target_id,
                            pointer_id: *pointer_id,
                            way: *way,
                        });
                    }
                }
            }
            None => {
                debug!("Pointer {:?} wasn't pressing anything", pointer_id);
            }
        }
    }
}
