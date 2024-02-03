use crate::character_types::*;
use bevy::prelude::*;
pub struct CharacterTypesPlugin;

impl Plugin for CharacterTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Character>();
        app.register_type::<MainCharacter>();
    }
}
