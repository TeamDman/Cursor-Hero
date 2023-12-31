use crate::plugins::character_plugin::Character;
use bevy::prelude::*;

use super::types::*;

pub fn toolbelt_spawn_setup_system(
    mut commands: Commands,
    character: Query<Entity, With<Character>>,
    mut writer: EventWriter<ToolbeltEvent>,
) {
    if let Ok(character_id) = character.get_single() {
        commands.entity(character_id).with_children(|c_commands| {
            let toolbelt = c_commands.spawn(ToolbeltBundle::default());
            writer.send(ToolbeltEvent::Populate(toolbelt.id()));
        });
    
        info!("Toolbelt setup complete");
    } else {
        unreachable!("Toolbelt setup system is configured to only run after the character is spawned.")
    }
    
}
