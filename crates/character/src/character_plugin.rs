use bevy::prelude::*;

use crate::character_appearance_plugin::CharacterAppearancePlugin;
use crate::character_spawning_plugin::CharacterSpawningPlugin;
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterAppearancePlugin);
        app.add_plugins(CharacterSpawningPlugin);
    }
}
