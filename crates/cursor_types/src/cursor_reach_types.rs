use bevy::prelude::*;

#[derive(Event, Debug, Reflect)]
pub enum CursorReachEvent {
    SetCursor { cursor_id: Entity, reach: f32 },
    SetCursorPercent { cursor_id: Entity, percent: f32 },
    SetCharacter { character_id: Entity, reach: f32 },
    SetCharacterPercent { character_id: Entity, percent: f32 },
    ResetCursor { cursor_id: Entity },
    ResetCharacter { character_id: Entity },
}
