use bevy::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_cursor_types::prelude::*;
use cursor_hero_math::prelude::Lerp;

use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintEvent;
use cursor_hero_toolbelt_types::toolbelt_types::Wheel;

pub struct CursorReachPlugin;

impl Plugin for CursorReachPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_reach_events);
        app.add_systems(Update, handle_sprint_events);
    }
}

fn handle_reach_events(
    mut reach_events: EventReader<CursorReachEvent>,
    character_query: Query<&Children, With<Character>>,
    mut cursor_query: Query<&mut Cursor>,
) {
    for event in reach_events.read() {
        match event {
            CursorReachEvent::SetCursor { cursor_id, reach } => {
                let Ok(mut cursor) = cursor_query.get_mut(*cursor_id) else {
                    warn!("cursor not found processing {:?}", event);
                    continue;
                };
                cursor.reach = *reach;
            }
            CursorReachEvent::SetCursorPercent { cursor_id, percent } => {
                let Ok(mut cursor) = cursor_query.get_mut(*cursor_id) else {
                    warn!("cursor not found processing {:?}", event);
                    continue;
                };
                cursor.reach = (cursor.default_reach, cursor.sprint_reach).lerp(*percent);
            }

            CursorReachEvent::SetCharacter {
                character_id,
                reach,
            } => {
                let Ok(character) = character_query.get(*character_id) else {
                    warn!("Character not found processing {:?}", event);
                    continue;
                };
                let mut found = false;
                for kid in character.iter() {
                    let Ok(mut cursor) = cursor_query.get_mut(*kid) else {
                        continue;
                    };
                    cursor.reach = *reach;
                    found = true;
                }
                if !found {
                    warn!("cursor not found processing {:?}", event);
                }
            }
            CursorReachEvent::SetCharacterPercent {
                character_id,
                percent,
            } => {
                let Ok(character) = character_query.get(*character_id) else {
                    warn!("Character not found processing {:?}", event);
                    continue;
                };
                let mut found = false;
                for kid in character.iter() {
                    let Ok(mut cursor) = cursor_query.get_mut(*kid) else {
                        continue;
                    };
                    cursor.reach = (cursor.default_reach, cursor.sprint_reach).lerp(*percent);
                    found = true;
                }
                if !found {
                    warn!("cursor not found processing {:?}", event);
                }
            }

            CursorReachEvent::ResetCursor { cursor_id } => {
                let Ok(mut cursor) = cursor_query.get_mut(*cursor_id) else {
                    warn!("cursor not found processing {:?}", event);
                    continue;
                };
                cursor.reach = cursor.default_reach;
            }
            CursorReachEvent::ResetCharacter { character_id } => {
                let Ok(character) = character_query.get(*character_id) else {
                    warn!("Character not found processing {:?}", event);
                    continue;
                };
                let mut found = false;
                for kid in character.iter() {
                    let Ok(mut cursor) = cursor_query.get_mut(*kid) else {
                        continue;
                    };
                    cursor.reach = cursor.default_reach;
                    found = true;
                }
                if !found {
                    warn!("cursor not found processing {:?}", event);
                }
            }
        }
    }
}

fn handle_sprint_events(
    mut reach_events: EventWriter<CursorReachEvent>,
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
                reach_events.send(CursorReachEvent::SetCharacterPercent {
                    character_id: *character_id,
                    percent: *throttle,
                });
            }
            SprintEvent::Stop { character_id } => {
                reach_events.send(CursorReachEvent::ResetCharacter {
                    character_id: *character_id,
                });
            }
        }
    }
}
