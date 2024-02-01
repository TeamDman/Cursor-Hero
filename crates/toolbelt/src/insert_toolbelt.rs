use bevy::prelude::*;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_character::character_plugin::MainCharacter;

use cursor_hero_toolbelt_types::types::*;

pub fn insert_toolbelt(
    mut commands: Commands,
    mut writer: EventWriter<PopulateToolbeltEvent>,
    fresh_characters: Query<(Entity, Option<&MainCharacter>), Added<Character>>,
) {
    for character in fresh_characters.iter() {
        let (character_id, is_main_character) = character;
        commands.entity(character_id).with_children(|c_commands| {
            let toolbelt = c_commands.spawn(ToolbeltBundle::default());
            if is_main_character.is_some() {
                writer.send(PopulateToolbeltEvent::Default {
                    toolbelt_id: toolbelt.id(),
                });
            }
        });

        info!("Toolbelt setup complete");
    }
}
