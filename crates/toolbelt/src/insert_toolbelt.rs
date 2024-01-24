use bevy::prelude::*;
use cursor_hero_character::character_plugin::Character;

use super::types::*;

pub fn insert_toolbelt(
    mut commands: Commands,
    mut writer: EventWriter<ToolbeltPopulateEvent>,
    fresh_characters: Query<Entity, Added<Character>>,
) {
    for character_id in fresh_characters.iter() {
        commands.entity(character_id).with_children(|c_commands| {
            let toolbelt = c_commands.spawn(ToolbeltBundle::default());
            writer.send(ToolbeltPopulateEvent::Default {
                toolbelt_id: toolbelt.id(),
            });
        });

        info!("Toolbelt setup complete");
    }
}
