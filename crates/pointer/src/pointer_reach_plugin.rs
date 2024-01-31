use bevy::prelude::*;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_math::Lerp;
use cursor_hero_pointer_types::prelude::*;

use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintEvent;
use cursor_hero_toolbelt_types::types::Wheel;

pub struct PointerReachPlugin;

impl Plugin for PointerReachPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_reach_events);
        app.add_systems(Update, handle_sprint_events);
    }
}

fn handle_reach_events(
    mut reach_events: EventReader<PointerReachEvent>,
    character_query: Query<&Children, With<Character>>,
    mut pointer_query: Query<&mut Pointer>,
) {
    for event in reach_events.read() {
        match event {
            PointerReachEvent::SetPointer { pointer_id, reach } => {
                let Ok(mut pointer) = pointer_query.get_mut(*pointer_id) else {
                    warn!("Pointer not found processing {:?}", event);
                    continue;
                };
                pointer.reach = *reach;
            }
            PointerReachEvent::SetPointerPercent {
                pointer_id,
                percent,
            } => {
                let Ok(mut pointer) = pointer_query.get_mut(*pointer_id) else {
                    warn!("Pointer not found processing {:?}", event);
                    continue;
                };
                pointer.reach = (pointer.default_reach, pointer.sprint_reach).lerp(*percent);
            }

            PointerReachEvent::SetCharacter {
                character_id,
                reach,
            } => {
                let Ok(character) = character_query.get(*character_id) else {
                    warn!("Character not found processing {:?}", event);
                    continue;
                };
                let mut found = false;
                for kid in character.iter() {
                    let Ok(mut pointer) = pointer_query.get_mut(*kid) else {
                        continue;
                    };
                    pointer.reach = *reach;
                    found = true;
                }
                if !found {
                    warn!("Pointer not found processing {:?}", event);
                }
            }
            PointerReachEvent::SetCharacterPercent {
                character_id,
                percent,
            } => {
                let Ok(character) = character_query.get(*character_id) else {
                    warn!("Character not found processing {:?}", event);
                    continue;
                };
                let mut found = false;
                for kid in character.iter() {
                    let Ok(mut pointer) = pointer_query.get_mut(*kid) else {
                        continue;
                    };
                    pointer.reach = (pointer.default_reach, pointer.sprint_reach).lerp(*percent);
                    found = true;
                }
                if !found {
                    warn!("Pointer not found processing {:?}", event);
                }
            }

            PointerReachEvent::ResetPointer { pointer_id } => {
                let Ok(mut pointer) = pointer_query.get_mut(*pointer_id) else {
                    warn!("Pointer not found processing {:?}", event);
                    continue;
                };
                pointer.reach = pointer.default_reach;
            }
            PointerReachEvent::ResetCharacter { character_id } => {
                let Ok(character) = character_query.get(*character_id) else {
                    warn!("Character not found processing {:?}", event);
                    continue;
                };
                let mut found = false;
                for kid in character.iter() {
                    let Ok(mut pointer) = pointer_query.get_mut(*kid) else {
                        continue;
                    };
                    pointer.reach = pointer.default_reach;
                    found = true;
                }
                if !found {
                    warn!("Pointer not found processing {:?}", event);
                }
            }
        }
    }
}

fn handle_sprint_events(
    mut reach_events: EventWriter<PointerReachEvent>,
    mut sprint_events: EventReader<SprintEvent>,
    character_query: Query<&Children, With<Character>>,
    toolbelt_query: Query<&Wheel, With<Character>>,
) {
    for event in sprint_events.read() {
        let character_id = match event {
            SprintEvent::Active { character_id, .. } => character_id,
            SprintEvent::Stop { character_id } => character_id,
        };
        let Ok(character) = character_query.get(*character_id) else {
            warn!("Character not found processing {:?}", event);
            continue;
        };
        let toolbelt_open = character.iter().any(|kid| {
            let Ok(toolbelt) = toolbelt_query.get(*kid) else {
                return false;
            };
            toolbelt.open
        });
        if toolbelt_open {
            // Toolbelt reach updates take priority over sprint reach updates
            continue;
        }
        match event {
            SprintEvent::Active {
                character_id,
                throttle,
            } => {
                reach_events.send(PointerReachEvent::SetCharacterPercent {
                    character_id: *character_id,
                    percent: *throttle,
                });
            }
            SprintEvent::Stop { character_id } => {
                reach_events.send(PointerReachEvent::ResetCharacter {
                    character_id: *character_id,
                });
            }
        }
    }
}
