use bevy::prelude::*;

#[derive(Event, Debug, Reflect)]
pub enum PointerReachEvent {
    SetPointer { pointer_id: Entity, reach: f32 },
    SetPointerPercent { pointer_id: Entity, percent: f32 },
    SetCharacter { character_id: Entity, reach: f32 },
    SetCharacterPercent { character_id: Entity, percent: f32 },
    ResetPointer { pointer_id: Entity },
    ResetCharacter { character_id: Entity },
}
