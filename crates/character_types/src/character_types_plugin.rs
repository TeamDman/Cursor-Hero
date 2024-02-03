use bevy::prelude::*;
use crate::character_types::*;
pub struct CharacterTypesPlugin;

impl Plugin for CharacterTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Character>();
        app.register_type::<MainCharacter>();
    }
}